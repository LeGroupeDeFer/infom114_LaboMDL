use diesel::prelude::*;
use diesel::MysqlConnection;
use regex::Regex;

use crate::database::models::prelude::*;
use crate::database::models::Entity;
use crate::database::tables::{
    capabilities_table, roles_capabilities_table, roles_table, users_roles_table,
};

use crate::database::schema::capabilities::dsl as capabilities;
use crate::database::schema::users::dsl::{self, users as table};

use crate::database;
use crate::lib::consequence::*;
use chrono::NaiveDateTime;
use diesel::dsl::count;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub address: Option<Address>,
    pub roles: Vec<Role>,
}

impl From<UserEntity> for User {
    fn from(ue: UserEntity) -> Self {
        let conn = database::connection(&database::url());
        Self {
            id: ue.id,
            firstname: ue.firstname.to_string(),
            lastname: ue.lastname.to_string(),
            email: ue.email.to_string(),
            address: ue.address.and_then(|address_id| {
                AddressEntity::by_id(&conn, &address_id)
                    .unwrap()
                    .and_then(|address_entity| Some(Address::from(address_entity)))
            }),
            roles: RelUserRoleEntity::get_roles_by_user(&conn, &ue)
                .unwrap()
                .drain(..)
                .map(|role_entity| Role::from(role_entity))
                .collect::<Vec<Role>>(),
        }
    }
}

impl UserEntity {
    /* ---------------------------------------- STATIC ---------------------------------------- */

    /// Constructor of `User` struct.
    /// Fetch a user in database based on its email field.
    pub fn by_email(conn: &MysqlConnection, email: &str) -> Consequence<Option<Self>> {
        table
            .filter(dsl::email.eq(email))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    // is_available_email :: (MysqlConnection, String) -> Boolean
    pub fn is_available_email(conn: &MysqlConnection, email: &str) -> Consequence<bool> {
        UserEntity::by_email(conn, email) // Result<User>
            .map(|_| true) // Result<bool>
            .or_else(|e| match e {
                Error::NotFound => Ok(true),
                other => Err(other),
            })
    }

    pub fn count_users(conn: &MysqlConnection, only_active: bool) -> Consequence<u64> {
        let mut query = table.select(count(dsl::id)).into_boxed();

        if only_active {
            query = query.filter(dsl::active.eq(only_active));
        }

        let count = query.first::<i64>(conn).map(|c: i64| c as u64)?;

        Ok(count)
    }

    pub fn count_connected(conn: &MysqlConnection) -> Consequence<u64> {
        let close_to_now: NaiveDateTime =
            chrono::offset::Local::now().naive_local() - chrono::Duration::minutes(15);

        Ok(UserEntity::all(conn)?
            .iter()
            .filter(|&user| user.last_connection >= close_to_now)
            .count() as u64)
    }

    /* --------------------------------------- DYNAMIC ---------------------------------------- */

    pub fn set_password(&mut self, password: &str) -> Consequence<&Self> {
        let hash = bcrypt::hash(&password, 8)?;
        self.password = hash;
        Ok(self)
    }

    pub fn activation_token(&self, conn: &MysqlConnection) -> Consequence<Option<TokenEntity>> {
        self.activation_token
            .and_then(|id| TokenEntity::by_id(conn, &id).transpose())
            .transpose()
    }

    pub fn recovery_token(&self, conn: &MysqlConnection) -> Consequence<Option<TokenEntity>> {
        self.recovery_token
            .and_then(|id| TokenEntity::by_id(conn, &id).transpose())
            .transpose()
    }

    pub fn refresh_token(&self, conn: &MysqlConnection) -> Consequence<Option<TokenEntity>> {
        self.refresh_token
            .and_then(|id| TokenEntity::by_id(conn, &id).transpose())
            .transpose()
    }

    pub fn get_last_id(conn: &MysqlConnection) -> Consequence<u32> {
        let found = table
            .select(dsl::id)
            .order(dsl::id.desc())
            .first::<u32>(conn)
            .optional()?;
        Ok(found.unwrap_or(0u32))
    }

    pub fn verify(&self, password: &str) -> Consequence<bool> {
        bcrypt::verify(password, &self.password).map(Ok)?
    }

    pub fn activate(&mut self, conn: &MysqlConnection) -> Consequence<&Self> {
        let mut token = self.activation_token(conn)??;
        token.verify(&token.hash)?;
        token.consume(conn)?;
        if self.recovery_token.is_none() {
            let recovery_token = TokenEntity::create_default(&*conn)?;
            self.recovery_token = Some(recovery_token.id);
        }
        if self.refresh_token.is_none() {
            let refresh_token = TokenEntity::create(&*conn, Some(&1209600), Some(&-1))?;
            self.refresh_token = Some(refresh_token.id);
        }
        self.active = true;
        self.update(conn)?;
        Ok(self)
    }

    pub fn data(&self) -> PublicUser {
        PublicUser {
            id: self.id,
            email: self.email.clone(),
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            address: self.address,
            phone: self.phone.clone(),
            creation_date: self.creation_date,
            last_connection: self.last_connection,
            active: self.active,
        }
    }

    /// Return a vector of `Role` struct
    pub fn get_roles(&self, conn: &MysqlConnection) -> Consequence<Vec<RoleEntity>> {
        let raw: Vec<Consequence<Option<RoleEntity>>> =
            RelUserRoleEntity::get_roles_by_user(&conn, &self)?
                .iter()
                .map(|r| RoleEntity::by_id(&conn, &r.id)) // FIXME - N query, ought to be 1 query instead
                .collect();
        let roles = raw
            .into_iter()
            .collect::<Consequence<Vec<Option<RoleEntity>>>>()?
            .into_iter()
            .collect::<Option<Vec<RoleEntity>>>()?;
        Ok(roles)
    }

    /// Get the capability of a user
    /// Return a vector of `models::roles::capability::Capability` struct
    pub fn get_capabilities(&self, conn: &MysqlConnection) -> Consequence<Vec<CapabilityEntity>> {
        let mut tab: Vec<CapabilityEntity> = Vec::new();
        let roles = self.get_roles(&conn)?;
        for r in roles {
            for c in r.capabilities(&conn)? {
                if !tab.contains(&c) {
                    tab.push(c);
                }
            }
        }

        Ok(tab)
    }

    pub fn has_capability(&self, conn: &MysqlConnection, capability: &str) -> bool {
        table
            .inner_join(users_roles_table.inner_join(
                roles_table.inner_join(roles_capabilities_table.inner_join(capabilities_table)),
            ))
            .filter(dsl::id.eq(self.id).and(capabilities::name.eq(capability)))
            .first::<(
                UserEntity,
                (
                    RelUserRoleEntity,
                    (RoleEntity, (RelRoleCapabilityEntity, CapabilityEntity)),
                ),
            )>(conn)
            .optional()
            .unwrap_or(None)
            .is_some()
    }

    /// Validate the fact that the email given
    ///
    /// * is a valid email
    /// * is issued from the unamur domain
    ///
    /// # Examples
    ///
    /// ```
    /// use unanimitylibrary::database::models::prelude::UserEntity;
    ///
    /// // valid
    /// assert!(UserEntity::check_if_email_is_unamur("guillaume.latour@student.unamur.be"));
    /// assert!(UserEntity::check_if_email_is_unamur("user.member@unamur.be"));
    ///
    /// // invalid
    /// assert!(!UserEntity::check_if_email_is_unamur("guillaume.latour.student.unamur.be"));
    /// assert!(!UserEntity::check_if_email_is_unamur("unamur@be"));
    /// ```
    pub fn check_if_email_is_unamur(email_address: &str) -> bool {
        let re = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
        re.is_match(email_address)
    }
}
