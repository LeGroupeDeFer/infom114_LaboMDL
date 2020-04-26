//! # Capability
//!
//! This module consist of the `Capability` struct, wich represent the database
//! columns of the table `capabilities`
//!
//! There is also the `CapabilityMinima` struct, which can be used to insert
//! data in the database.
//!
//! You can also find the list of the available capabilities

use crate::database::models::Entity;
use crate::database::models::result::Result;
use crate::database::schema::capabilities;
use crate::database::schema::capabilities::dsl::capabilities as table;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

/// The `Capability` struct is the usable type for what's in the database
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "capabilities"]
pub struct Capability {
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

impl Entity for Capability {

    type Minima = CapabilityMinima;

    /// Constructor of `Capability` from a role id
    fn by_id(conn: &MysqlConnection, id: &u32) ->Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    /// Get all the capabilities in the database in an array of `Capability`
    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    /// Add a new capability in database.
    ///
    /// This function should not be called outside a "seeder" or an update
    /// mechanism for the application because the capabilities will be
    /// hardcoded to check each and every feature's access, so it makes no
    /// sense if one can add capability dynamically.
    fn insert(conn: &MysqlConnection, minima: &CapabilityMinima) -> Result<Either<Self, Self>> {
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
    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        table
            .filter(capabilities::name.eq(&minima.name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::capabilities::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

}

impl Capability {
    /// Constructor of `Capability` from a role name
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        table
            .filter(capabilities::name.eq(&name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }
}

/// All the capabilities of the application
pub const CAPABILITIES: [&str; 4] = [
    "role:manage",
    "post:create",
    "user:manage",
    "user:manage_role",
];
