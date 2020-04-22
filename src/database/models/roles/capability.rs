//! # Capability
//!
//! This module consist of the `Capability` struct, wich represent the database
//! columns of the table `capabilities`
//!
//! There is also the `CapabilityMinima` struct, which can be used to insert
//! data in the database.
//!
//! You can also find the list of the available capabilities

use crate::database::schema::capabilities;
use crate::database::schema::capabilities::dsl::capabilities as table;
use crate::database::Data;
use diesel::prelude::*;
use diesel::MysqlConnection;

/// The `Capability` struct is the usable type for what's in the database
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl Capability {
    /// Constructor of `Capability` from a role id
    pub fn by_id(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    /// Constructor of `Capability` from a role name
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        table
            .filter(capabilities::name.eq(&name))
            .first::<Self>(conn)
            .ok()
    }

    /// Get all the capabilities in the database in an array of `Capability`
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    /// Constructor of `Capability` that fits the `minima` given.
    pub fn select_minima(conn: &MysqlConnection, minima: &CapabilityMinima) -> Option<Self> {
        Self::by_name(&conn, &minima.name)
    }

    /// Add a new capability in database.
    ///
    /// This function should not be called outside a "seeder" or an update
    /// mechanism for the application because the capabilities will be
    /// hardcoded to check each and every feature's access, so it makes no
    /// sense if one can add capability dynamically.
    pub fn insert_minima(conn: &MysqlConnection, minima: &CapabilityMinima) -> Data<Self> {
        if let Some(past) = Self::select_minima(conn, minima) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::select_minima(conn, minima)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }
}

/// All the capabilities of the application
pub const CAPABILITIES: [&str; 4] = [
    "role:manage",
    "user:manage_role",
    "tag:update",
    "tag:delete",
];
