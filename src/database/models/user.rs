use super::address::Address;
use crate::database::schema::users;
use crate::database::schema::users::dsl::users as table;
use crate::database::Connection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use either::*;
use regex::Regex;
use rocket_contrib::json::JsonValue;

/* ---------------------------------- User --------------------------------- */

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
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

    // from :: (Connection, Integer) -> Option<User>
    pub fn from(conn: &Connection, id: &u32) -> Option<Self> {
        table.find(id).first::<User>(&**conn).ok()
    }

    // all :: (Connection) -> Vec<User>
    pub fn all(conn: &Connection) -> Vec<Self> {
        table.load(&**conn).unwrap_or(vec![])
    }

    // by_email :: (Connection, String) -> Option<User>
    pub fn by_email(conn: &Connection, email: &str) -> Option<User> {
        if let Ok(user) = table.filter(users::email.eq(email)).first(&**conn) {
            Some(user)
        } else {
            None
        }
    }

    // is_available_email :: (Connection, String) -> Boolean
    pub fn is_available_email(conn: &Connection, email: &str) -> bool {
        User::by_email(conn, email).is_none()
    }

    // is_unamur_email :: String -> Boolean
    pub fn is_unamur_email(email: &str) -> bool {
        let re: Regex = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
        re.is_match(email)
    }

    // select_minima :: (Connection, UserMinima) -> Option<User>
    pub fn select_minima(conn: &Connection, minima: &UserMinima) -> Option<Self> {
        table
            .filter(users::email.eq(minima.email.clone()))
            .first::<User>(&**conn)
            .ok()
    }

    // insert_minima :: (Connection, UserMinima) -> Either<User, User>
    pub fn insert_minima(conn: &Connection, minima: &UserMinima) -> Either<Self, Self> {
        if let Some(past) = User::select_minima(conn, minima) {
            Left(past)
        } else {
            let mut inserted = minima.clone();
            inserted.password = bcrypt::hash(&minima.password, 8).expect("Unable to hash password");
            diesel::insert_into(table)
                .values(inserted)
                .execute(&**conn)
                .expect("Error inserting address");
            Right(
                User::select_minima(conn, minima)
                    .expect("User insertion succeeded but could not be retreived"),
            )
        }
    }

    /* ------------------------------ DYNAMIC ------------------------------ */

    pub fn cookie(&self) -> String {
        self.id.to_string()
    }

    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).expect("Fatal: BCrypt error")
    }

    pub fn activate(&self, conn: &Connection) {
        use crate::database::schema::users::dsl::*;
        diesel::update(users.filter(id.eq(self.id)))
            .set((active.eq(true), token.eq(None: Option<String>)))
            .execute(&**conn)
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
