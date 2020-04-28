extern crate rand;
use std::convert::TryFrom;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;

use crate::lib::consequence::*;
use crate::database::models::prelude::*;
use crate::database::models::Entity;

use crate::database::schema::tokens;
use crate::database::schema::tokens::dsl::{self, tokens as table};


#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Debug)]
#[table_name = "tokens"]
#[changeset_options(treat_none_as_null = "true")]
pub struct TokenEntity {
    pub id: u32,
    pub hash: String,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
    pub count: i32,
    pub consumed: bool,
}


#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "tokens"]
pub struct TokenMinima {
    pub hash: String,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
    pub count: i32,
}


impl Entity for TokenEntity {

    type Minima = TokenMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(tokens::hash.eq(minima.hash.clone()))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        use crate::database::schema::tokens::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

}



impl From<TokenEntity> for String {
    fn from(token: TokenEntity) -> String {
        token.hash
    }
}

impl fmt::Display for TokenEntity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}
