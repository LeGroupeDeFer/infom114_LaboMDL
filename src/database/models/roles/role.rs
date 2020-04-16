//! # Role module
//!
//! Here will be grouped every structs that allows the representation of the
//! table `roles` and what is needed by rust to insert data in it.

use crate::database::Data;

use crate::database::models::roles::capability::Capability;
use crate::database::models::roles::role_capability::RelRoleCapability;

use crate::database::schema::roles;
use crate::database::schema::roles::dsl::roles as table;

use crate::database::schema::roles_capabilities::{
    self, dsl::roles_capabilities as table_roles_capabilities,
};

use diesel::prelude::*;
use diesel::MysqlConnection;

/// The struct `Role` is the perfect representation of the data that can be hold
/// in the `roles` table.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "roles"]
pub struct Role {
    pub id: u32,
    pub name: String,
    pub color: String,
}

/// The struct `Roleminima` is needed by rust to perform an insert in the database
/// because the role id is auto incremented, we do not know it before inserting data.
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "roles"]
pub struct RoleMinima {
    pub name: String,
    pub color: String,
}

impl Role {
    /* ------------------------------- STATIC ------------------------------ */

    /// Constructor based on the role id.
    pub fn by_id(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    /// Get the Capability record that fits the role name given.
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        table.filter(roles::name.eq(name)).first::<Self>(conn).ok()
    }

    /// Fetch and return all the roles present in database as a `Role` vector
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    /// Insert data stored in the `RoleMinima` struct given in parameter inside the
    /// database1
    pub fn insert_minima(conn: &MysqlConnection, minima: &RoleMinima) -> Data<Self> {
        if let Some(past) = Self::by_name(conn, &minima.name) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::by_name(conn, &minima.name)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }

    /* ------------------------------- NOT SO STATIC ------------------------------ */

    /// Get the capabilities linked to a role
    pub fn capabilities(&self, conn: &MysqlConnection) -> Vec<Capability> {
        RelRoleCapability::get_capabilities_for_role(&conn, &self)
    }

    /// Clear the database stored capabilities linked to this role
    pub fn clear_capabilities(&self, conn: &MysqlConnection) {
        diesel::delete(table_roles_capabilities.filter(roles_capabilities::role_id.eq(self.id)))
            .execute(conn)
            .unwrap();
    }

    /// Update the role database informations
    pub fn update(&self, conn: &MysqlConnection, minima: &RoleMinima) {
        diesel::update(self)
            .set((roles::name.eq(&minima.name), roles::color.eq(&minima.color)))
            .execute(conn)
            .unwrap();
    }

    /// Delete the role in database
    /// This will first remove all capabilities linked to this role
    pub fn delete(self, conn: &MysqlConnection) {
        self.clear_capabilities(conn);
        diesel::delete(&self).execute(conn).unwrap();
    }
}
