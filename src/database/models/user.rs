use super::address::Address;
use crate::database::models::roles;
use crate::database::schema::users;
use crate::database::schema::users::dsl::users as table;
use crate::database::Data;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;
use regex::Regex;
use rocket_contrib::json::JsonValue;

/* ---------------------------------- User --------------------------------- */

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

    pub token: Option<String>,
    pub active: bool,
}

impl User {
    /* ------------------------------- STATIC ------------------------------ */

    // from :: (DBConnection, Integer) -> Option<User>
    pub fn from(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    // all :: (MysqlConnection) -> Vec<User>
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    /// Constructor of `User` struct.
    /// Fetch a user in database based on its email field.
    pub fn by_email(conn: &MysqlConnection, email: &str) -> Option<User> {
        if let Ok(user) = table.filter(users::email.eq(email)).first(conn) {
            Some(user)
        } else {
            None
        }
    }

    // is_available_email :: (MysqlConnection, String) -> Boolean
    pub fn is_available_email(conn: &MysqlConnection, email: &str) -> bool {
        User::by_email(conn, email).is_none()
    }

    // is_unamur_email :: String -> Boolean
    pub fn is_unamur_email(email: &str) -> bool {
        let re: Regex = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
        re.is_match(email)
    }

    // select_minima :: (MysqlConnection, UserMinima) -> Option<User>
    pub fn select_minima(conn: &MysqlConnection, minima: &UserMinima) -> Option<Self> {
        table
            .filter(users::email.eq(minima.email.clone()))
            .first::<User>(conn)
            .ok()
    }

    // insert_minima :: (MysqlConnection, UserMinima) -> Either<User, User>
    pub fn insert_minima(conn: &MysqlConnection, minima: &UserMinima) -> Data<Self> {
        if let Some(past) = User::select_minima(conn, minima) {
            Data::Existing(past)
        } else {
            let mut inserted = minima.clone();
            inserted.password = bcrypt::hash(&minima.password, 8).expect("Unable to hash password");
            diesel::insert_into(table)
                .values(inserted)
                .execute(conn)
                .expect("Error inserting address");
            Data::Inserted(
                User::select_minima(conn, minima)
                    .expect("User insertion succeeded but could not be retreived"),
            )
        }
    }

    pub fn get_last_id(conn: &MysqlConnection) -> u32 {
        use crate::database::schema::users::dsl::*;
        table
            .select(id)
            .order(id.desc())
            .first::<u32>(conn)
            .ok()
            .unwrap_or(0u32)
    }

    /* ------------------------------ DYNAMIC ------------------------------ */

    pub fn cookie(&self) -> String {
        self.id.to_string()
    }

    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).expect("Fatal: BCrypt error")
    }

    pub fn activate(&self, conn: &MysqlConnection) {
        use crate::database::schema::users::dsl::*;
        diesel::update(users.filter(id.eq(self.id)))
            .set((active.eq(true), token.eq(None: Option<String>)))
            .execute(conn)
            .expect(&format!("Error updating user #{}", self.id));
    }

    pub fn data(&self) -> JsonValue {
        json!({
            "id": self.id,
            "firstname": self.firstname,
            "lastname": self.lastname,
            "phone": self.phone,
            "creation_date": self.creation_date,
            "last_connection": self.last_connection
        })
    }

    /// Get the roles of a user
    /// Return a vector of `models::roles::role::Role` struct
    pub fn get_roles(&self, conn: &MysqlConnection) -> Vec<roles::role::Role> {
        roles::user_role::RelUserRole::get_roles_from_user(&conn, &self)
            .iter()
            .map(|r| roles::role::Role::by_id(&conn, &r.id))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect::<_>()
    }

    /// Get the capabilities of a user
    /// Return a vector of `models::roles::capability::Capability` struct
    pub fn get_capabilities(&self, conn: &MysqlConnection) -> Vec<roles::capability::Capability> {
        let mut tab: Vec<roles::capability::Capability> = Vec::new();
        let roles = self.get_roles(&conn);
        for r in roles {
            for c in r.capabilities(&conn) {
                if !tab.contains(&c) {
                    tab.push(c);
                }
            }
        }

        tab
    }

    pub fn get_id_by_token(conn: &MysqlConnection, token: &str) -> Option<u32> {
        if let Ok(user_id) = table.filter(users::token.eq(token)).select(users::id).first(conn) {
            Some(user_id)
        } else {
            None
        }
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

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct UserMinima {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<u32>,
    pub phone: Option<String>,
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
        }
    }
}
