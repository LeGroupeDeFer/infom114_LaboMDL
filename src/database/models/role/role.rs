//! # Role module
//!
//! Here will be grouped every structs that allows the representation of the
//! table `roles` and what is needed by rust to insert data in it.

use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::models::result::Result;
use crate::database::models::prelude::{Capability};

use super::entity::Role;
use crate::database::models::role::RelRoleCapability;
use crate::database::schema::roles::dsl::{self, roles as table};
use crate::database::schema::roles_capabilities::dsl::{
    self as rc_dsl,
    roles_capabilities as rc_table
};

impl Role {

    /* ------------------------------- STATIC ------------------------------ */

    /// Get the Capability record that fits the role name given.
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        table
            .filter(dsl::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    /* ------------------------------- NOT SO STATIC ------------------------------ */

    /// Get the capability linked to a role
    pub fn capabilities(&self, conn: &MysqlConnection) -> Result<Vec<Capability>> {
        Ok(RelRoleCapability::get_capabilities_for_role(&conn, &self)?)
    }

    /// Clear the database stored capabilities linked to this role
    pub fn clear_capabilities(&self, conn: &MysqlConnection) {
        diesel::delete(rc_table.filter(rc_dsl::role_id.eq(self.id)))
            .execute(conn)
            .unwrap();
    }

    /// Return all the roles with the corresponding capability as an array
    /// of `RoleCapabilities`
    pub fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        Ok(Role::all(conn)?
            .iter()
            .map(|r| Self::by_role(conn, &r))
            .collect::<Vec<Self>>())
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn by_role_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        Ok(Role::by_name(conn, name)?.map(|r| Self::by_role(conn, &r)))
    }

    /// Constructor of `RoleCapabilities` based on a `role::Role` object
    fn by_role(conn: &MysqlConnection, r: &Role) -> Self {
        Self {
            id: r.id,
            name: r.name.to_string(),
            color: r.color.to_string(),
            //capabilities: RelRoleCapability::get_capabilities_for_role(&conn, &r),
        }
    }
}
