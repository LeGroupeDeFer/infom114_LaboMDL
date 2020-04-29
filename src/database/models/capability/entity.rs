//! # Capability
//!
//! This module consist of the `Capability` struct, wich represent the database
//! columns of the table `capability`
//!
//! There is also the `CapabilityMinima` struct, which can be used to insert
//! data in the database.
//!
//! You can also find the list of the available capability

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::CapabilityData;
use crate::database::models::Entity;
use crate::database::schema::capabilities;
use crate::database::schema::capabilities::dsl::{self, capabilities as table};
use crate::lib::consequence::Consequence;

/// The `Capability` struct is the usable type for what's in the database
#[derive(
    Identifiable,
    Queryable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[table_name = "capabilities"]
pub struct CapabilityEntity {
    pub id: u32,
    pub name: String,
}

/// The `CapabilityMinima` struct is only used while inserting a new capability
/// in the database
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "capabilities"]
pub struct CapabilityMinima {
    pub name: String,
}

impl From<&CapabilityData> for CapabilityMinima {
    fn from(capability: &CapabilityData) -> Self {
        Self {
            name: capability.name.to_string(),
        }
    }
}

impl Entity for CapabilityEntity {
    type Minima = CapabilityMinima;

    /// Constructor of `Capability` from a role id
    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    /// Get all the capability in the database in an array of `Capability`
    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    /// Add a new capability in database.
    ///
    /// This function should not be called outside a "seeder" or an update
    /// mechanism for the application because the capability will be
    /// hardcoded to check each and every feature's access, so it makes no
    /// sense if one can add capability dynamically.
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

    /// Constructor of `Capability` that fits the `minima` given.
    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(dsl::name.eq(&minima.name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self)
            .set(self)
            .execute(conn)
            .map(|_| self)
            .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}
