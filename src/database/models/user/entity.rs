use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;
use regex::Regex;
use either::*;

use crate::database::models::prelude::*;
use crate::database::models::Entity;

use crate::database::schema::users;
use crate::database::schema::users::dsl::{self, users as table};

use crate::database::models::address::Address;


// We can't have the `activation_token` and `recovery_token` fks in Diesel as these are 2 separate
// foreign keys for the same table which is not supported by Diesel
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[belongs_to(Address, foreign_key = "address")]
#[table_name = "users"]
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


impl Entity for User {

    type Minima = UserMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
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

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        table
            .filter(dsl::email.eq(minima.email.clone()))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::users::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

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
