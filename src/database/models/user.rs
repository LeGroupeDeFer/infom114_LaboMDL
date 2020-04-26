use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;
use regex::Regex;
use either::*;

use super::Entity;
use super::address::Address;
use super::token::Token;
use super::result::*;

use crate::database::schema::users;
use crate::database::schema::users::dsl::users as table;
use crate::database::models::roles::{
    user_role::RelUserRole,
    role::Role,
    capability::Capability
};


// We can't have the `activation_token` and `recovery_token` fks in Diesel as these are 2 separate
// foreign keys for the same table which is not supported by Diesel
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[belongs_to(Address, foreign_key = "address")]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,

    pub address: Option<u32>,
    pub phone: Option<String>,

    pub creation_date: NaiveDateTime,
    pub last_connection: NaiveDateTime,

    pub activation_token: Option<u32>,
    pub recovery_token: Option<u32>,
    pub refresh_token: Option<u32>,

    pub active: bool,
}

impl Entity for User {

    type Minima = UserMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        table
            .filter(users::email.eq(minima.email.clone()))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        if !is_valid_email(&minima.email) {
            return Err(UserError::InvalidEmail)?;
        }

        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            let mut inserted = minima.clone();
            inserted.password = bcrypt::hash(&minima.password, 8)?;
            diesel::insert_into(table).values(inserted).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::users::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

}

impl User {

    /* ---------------------------------------- STATIC ---------------------------------------- */

    /// Constructor of `User` struct.
    /// Fetch a user in database based on its email field.
    pub fn by_email(conn: &MysqlConnection, email: &str) -> Result<Option<Self>> {
        table
            .filter(users::email.eq(email))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    // is_available_email :: (MysqlConnection, String) -> Boolean
    pub fn is_available_email(conn: &MysqlConnection, email: &str) -> Result<bool> {
        User::by_email(conn, email) // Result<User>
            .map(|_| true)          // Result<bool>
            .or_else(|e| match e {
                Error::NotFound => Ok(true),
                other => Err(other)
            })
    }

    /* --------------------------------------- DYNAMIC ---------------------------------------- */

    pub fn set_password(&mut self, password: &str) -> Result<&Self> {
        let hash = bcrypt::hash(&password, 8)?;
        self.password = hash;
        Ok(self)
    }

    pub fn activation_token(&self, conn: &MysqlConnection) -> Result<Option<Token>> {
        self.activation_token.and_then(|id| Token::by_id(conn, &id).transpose()).transpose()
    }

    pub fn recovery_token(&self, conn: &MysqlConnection) -> Result<Option<Token>> {
        self.recovery_token.and_then(|id| Token::by_id(conn, &id).transpose()).transpose()
    }

    pub fn refresh_token(&self, conn: &MysqlConnection) -> Result<Option<Token>> {
        self.refresh_token.and_then(|id| Token::by_id(conn, &id).transpose()).transpose()
    }

    pub fn get_last_id(conn: &MysqlConnection) -> Result<u32> {
        use crate::database::schema::users::dsl::*;
        let found = table
            .select(id)
            .order(id.desc())
            .first::<u32>(conn)
            .optional()?;
        Ok(found.unwrap_or(0u32))
    }

    pub fn verify(&self, password: &str) -> Result<bool> {
        bcrypt::verify(password, &self.password).map(Ok)?
    }

    pub fn activate(&mut self, conn: &MysqlConnection) -> Result<&Self> {
        let mut token = self.activation_token(conn)??;
        token.verify(&token.hash)?;
        token.consume(conn)?;
        if self.recovery_token.is_none() {
            let recovery_token = Token::create_default(&*conn)?;
            self.recovery_token = Some(recovery_token.id);
        }
        if self.refresh_token.is_none() {
            let refresh_token = Token::create(
                &*conn,
                Some(&1209600),
                Some(&-1)
            )?;
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
            active: self.active
        }
    }

    /// Get the roles of a user
    /// Return a vector of `Role` struct
    pub fn get_roles(&self, conn: &MysqlConnection) -> Result<Vec<Role>> {
        let raw: Vec<Result<Option<Role>>> =
            RelUserRole::get_roles_from_user(&conn, &self)?
                .iter()
                .map(|r| Role::by_id(&conn, &r.id)) // FIXME - N query, ought to be 1 query instead
                .collect();
        let roles = raw
            .into_iter().collect::<Result<Vec<Option<Role>>>>()?
            .into_iter().collect::<Option<Vec<Role>>>()?;
        Ok(roles)
    }

    /// Get the capabilities of a user
    /// Return a vector of `models::roles::capability::Capability` struct
    pub fn get_capabilities(&self, conn: &MysqlConnection) -> Result<Vec<Capability>> {
        let mut tab: Vec<Capability> = Vec::new();
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

    /// Validate the fact that the email given
    ///
    /// * is a valid email
    /// * is issued from the unamur domain
    ///
    /// # Examples
    ///
    /// ```
    /// use unanimitylibrary::database::models::user::User;
    ///
    /// // valid
    /// assert!(User::check_if_email_is_unamur("guillaume.latour@student.unamur.be"));
    /// assert!(User::check_if_email_is_unamur("user.member@unamur.be"));
    ///
    /// // invalid
    /// assert!(!User::check_if_email_is_unamur("guillaume.latour.student.unamur.be"));
    /// assert!(!User::check_if_email_is_unamur("unamur@be"));
    /// ```
    pub fn check_if_email_is_unamur(email_address: &str) -> bool {
        let re = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
        re.is_match(email_address)
    }
}

// ---------------------------------------------------------------------------------------- Minima

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct UserMinima {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<u32>,
    pub phone: Option<String>,
    pub activation_token: Option<u32>,
    pub refresh_token: Option<u32>,
    pub recovery_token: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: u32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,

    pub address: Option<u32>,
    pub phone: Option<String>,

    pub creation_date: NaiveDateTime,
    pub last_connection: NaiveDateTime,

    pub active: bool,
}

impl Clone for UserMinima {
    fn clone(&self) -> UserMinima {
        UserMinima {
            email: self.email.clone(),
            password: self.password.clone(),
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            address: self.address.clone(),
            phone: self.phone.clone(),
            activation_token: self.activation_token.clone(),
            recovery_token: self.recovery_token.clone(),
            refresh_token: self.refresh_token.clone()
        }
    }
}

// ------------------------------------------------------------------------------ Helper Functions

fn is_valid_email(email: &str) -> bool {
    // Simply checks if the email is an UNamur email, other tests might be added later on
    let re: Regex = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
    re.is_match(email)
}
