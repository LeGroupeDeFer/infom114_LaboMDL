//! # Roles module
//!
//! This module groups every models related to role management

pub mod capability;
pub mod forms;
pub mod role;
pub mod role_capability;

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
/// a capbility)
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
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        let roles = role::Role::all(conn);

        roles
            .iter()
            .map(|r| Self::from_role(conn, &r))
            .collect::<Vec<RoleCapabilities>>()
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn from_role_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        role::Role::from_name(conn, name).map(|r| Self::from_role(conn, &r))
    }

    /// Constructor of `RoleCapabilities` based on a `role::Role` object
    fn from_role(conn: &MysqlConnection, r: &role::Role) -> Self {
        RoleCapabilities {
            id: r.id,
            name: r.name.to_string(),
            color: r.color.to_string(),
            capabilities: role_capability::RelRoleCapability::get_capabilities_for_role(&conn, &r),
        }
    }
}
