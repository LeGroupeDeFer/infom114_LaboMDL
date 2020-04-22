extern crate rand;
use super::result::*;
use super::result::token::Error as TokenError;
use super::Entity;
use crate::database::schema::tokens;
use crate::database::schema::tokens::dsl::tokens as table;
use std::convert::TryFrom;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;


#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Clone, Debug)]
#[table_name = "tokens"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Token {
    pub id: u32,
    pub hash: String,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
    pub count: i32,
    pub consumed: bool,
}

impl Entity for Token {

    type Minima = TokenMinima;

    fn of(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        table
            .filter(tokens::hash.eq(minima.hash.clone()))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::tokens::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

}

impl Token {

    /* ---------------------------------------- STATIC ---------------------------------------- */

    pub fn by_hash(conn: &MysqlConnection, hash: &str) -> Result<Option<Self>> {
        table
            .filter(tokens::hash.eq(hash))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    pub fn create(
        conn: &MysqlConnection,
        lifetime: Option<&u32>,
        count: Option<&i32>
    ) -> Result<Self> {
        let hash: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let creation_date = Utc::now().naive_local();
        let expiration_date = lifetime
            .map(|seconds| Duration::seconds(*seconds as i64))
            .and_then(|duration| creation_date.checked_add_signed(duration));

        let minima = TokenMinima {
            hash,
            creation_date,
            expiration_date,
            count: *count.unwrap_or(&(1 as i32)),
        };

        Self::insert(conn, &minima).and_then(|insertion| {
            insertion.either(
                |_| Err(Error::from(TokenError::Collision)),
                |right| Ok(right)
            )
        })
    }

    pub fn create_default(conn: &MysqlConnection) -> Result<Self> {
        Self::create(conn, None, None)
    }

    /* --------------------------------------- DYNAMIC ---------------------------------------- */

    pub fn renew(
        &mut self,
        conn: &MysqlConnection,
        lifetime: Option<&u32>,
        count: Option<&i32>,
    ) -> Result<&Self> {
        let hash = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let creation_date = Utc::now().naive_local();

        let expiration_date = self.expiration_date.map(|expiration: NaiveDateTime| -> Result<NaiveDateTime> {
            let new_lifetime = lifetime.map(|v| *v as i64).unwrap_or(
                expiration.timestamp() - self.creation_date.timestamp()
            );
            creation_date.checked_add_signed(Duration::seconds(new_lifetime)).map(Ok)?
        }).transpose()?;

        self.hash = hash;
        self.creation_date = creation_date;
        self.expiration_date = expiration_date;
        self.count = *count.unwrap_or(&self.count);
        self.consumed = *count.unwrap_or(&self.count) == 0;

        self.update(conn)?;
        Ok(self)
    }

    pub fn revoke(&mut self, conn: &MysqlConnection) -> Result<&Self> {
        self.count = 0;
        self.consumed = true;
        self.expiration_date = Some(Utc::now().naive_local());

        self.update(conn)?;
        Ok(self)
    }

    pub fn valid(&self) -> bool {
        !self.consumed && !self.expired()
    }

    pub fn expired(&self) -> bool {
        if let Some(date) = self.expiration_date {
            date.timestamp() < Utc::now().timestamp()
        } else {
            false
        }
    }

    pub fn consume(&mut self, conn: &MysqlConnection) -> Result<&Self> {
        if self.consumed {
            Err(Error::from(TokenError::Consumed))
        } else if self.expired() {
            Err(Error::from(TokenError::Expired))
        } else if self.count == -1 {
            Ok(self)
        } else {
            self.count = if self.count > 0 { self.count - 1 } else { self.count };
            self.consumed = self.count == 0;

            self.update(conn)?;
            Ok(self)
        }
    }

    pub fn verify(&self, hash: &str) -> Result<&Self> {
        if self.expired() {
            Err(TokenError::Expired)?
        } else if self.consumed {
            Err(TokenError::Consumed)?
        } else if self.hash != hash {
            Err(TokenError::InvalidHash)?
        } else {
            Ok(self)
        }
    }

    pub fn vouch(&mut self, conn: &MysqlConnection, hash: &str) -> Result<&Self> {
        self.verify(hash)?;
        self.consume(conn)
    }

    pub fn lifespan(&self) -> u32 {
        self.expiration_date.map(|expires| {
            // The duration for a token creation is a u32 which implies that expiration - creation
            // can't overflow a u32, hence the unwrap()
            u32::try_from(expires.timestamp() - self.creation_date.timestamp()).unwrap()
        }).unwrap_or(u32::max_value())
    }

    pub fn ttl(&self) -> u32 {
        let now = Utc::now().naive_local();
        self.expiration_date.map(|expires| {
            // Same logic as in lifespan for the unwrap
            u32::try_from(expires.timestamp() - now.timestamp()).unwrap()
        }).unwrap_or(u32::max_value())
    }
}

impl From<Token> for String {
    fn from(token: Token) -> String {
        token.hash
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "tokens"]
pub struct TokenMinima {
    pub hash: String,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
    pub count: i32,
}
