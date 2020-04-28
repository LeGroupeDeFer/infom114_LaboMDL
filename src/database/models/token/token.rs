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
use super::entity::*;


impl TokenEntity {

    /* ---------------------------------------- STATIC ---------------------------------------- */

    pub fn by_hash(conn: &MysqlConnection, hash: &str) -> Consequence<Option<Self>> {
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
    ) -> Consequence<Self> {
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

    pub fn create_default(conn: &MysqlConnection) -> Consequence<Self> {
        Self::create(conn, None, None)
    }

    /* --------------------------------------- DYNAMIC ---------------------------------------- */

    pub fn renew(
        &mut self,
        conn: &MysqlConnection,
        lifetime: Option<&u32>,
        count: Option<&i32>,
    ) -> Consequence<&Self> {
        let hash = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let creation_date = Utc::now().naive_local();

        let expiration_date = self.expiration_date.map(|expiration: NaiveDateTime| -> Consequence<NaiveDateTime> {
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

    pub fn revoke(&mut self, conn: &MysqlConnection) -> Consequence<&Self> {
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

    pub fn consume(&mut self, conn: &MysqlConnection) -> Consequence<&Self> {
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

    pub fn verify(&self, hash: &str) -> Consequence<&Self> {
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

    pub fn vouch(&mut self, conn: &MysqlConnection, hash: &str) -> Consequence<&Self> {
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