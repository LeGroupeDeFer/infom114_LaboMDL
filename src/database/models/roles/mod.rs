//! # Roles module
//!
//! This module groups every models related to role management

pub mod capability;
pub mod forms;
pub mod role;
pub mod role_capability;
pub mod user_role;

use crate::database::models::Entity;
use crate::database::models::result::Result;
use crate::database::models::roles::capability::Capability;

use diesel::MysqlConnection;

/// This `RoleCapabilities` struct is not a real model because it's not a
/// fair representation of what's in the database.
/// In fact this struct is the concatenation of the `role::Role` and the
/// `capability::Capability` structs.
///
/// It's meant to be ease the use of the mentionned structs : instead of
/// having to manipulate three structs (you'll have to get through the
/// `role_capability::RoleCapability` struct to correctly link a role to
/// a capability)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RoleCapabilities {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub capabilities: Vec<Capability>,
}

impl RoleCapabilities {
    /// Return all the roles with the corresponding capabilities as an array
    /// of `RoleCapabilities`
    pub fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        let roles = role::Role::all(conn)?;

        let rc: Result<Vec<_>> = roles
            .into_iter()
            .map(|r| Self::by_role(conn, &r))
            .collect();
        Ok(rc?)
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn by_role_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        let role = role::Role::by_name(conn, name)?;
        role.map(|r| Self::by_role(conn, &r)).transpose()
    }

    /// Constructor of `RoleCapabilities` based on a `role::Role` object
    fn by_role(conn: &MysqlConnection, r: &role::Role) -> Result<Self> {
        let caps = role_capability::RelRoleCapability::get_capabilities_for_role(&conn, &r)?;
        Ok(RoleCapabilities {
            id: r.id,
            name: r.name.to_string(),
            color: r.color.to_string(),
            capabilities: caps,
        })
    }
}
