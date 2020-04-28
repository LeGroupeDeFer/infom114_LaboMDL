//! # Role module
//!
//! Here will be grouped every structs that allows the representation of the
//! table `roles` and what is needed by rust to insert data in it.

use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::models::prelude::*;
use crate::lib::consequence::Consequence;

use crate::database;
use crate::database::schema::roles::dsl::{self, roles as table};
use crate::database::schema::roles_capabilities::dsl::{
    self as rc_dsl, roles_capabilities as rc_table,
};

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
pub struct Role {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub capabilities: Vec<Capability>,
}

impl Role {
    /// Return all the roles with the corresponding capabilities as an array
    /// of `RoleCapabilities`
    pub fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(RoleEntity::all(conn)?
            .drain(..)
            .map(|r| Self::from(r))
            .collect::<Vec<Self>>())
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn by_role_name(conn: &MysqlConnection, name: &str) -> Consequence<Option<Self>> {
        Ok(RoleEntity::by_name(conn, name)?.map(|r| Self::from(r)))
    }
}

impl From<RoleEntity> for Role {
    fn from(re: RoleEntity) -> Self {
        let conn = database::connection(&database::url());
        Self {
            id: re.id,
            name: re.name.to_string(),
            color: re.color.to_string(),
            capabilities: RelRoleCapabilityEntity::get_capabilities_for_role(&conn, &re)
                .unwrap()
                .drain(..)
                .map(|capability_entity| Capability::from(capability_entity))
                .collect::<Vec<Capability>>(),
        }
    }
}

impl RoleEntity {
    /* ------------------------------- STATIC ------------------------------ */

    /// Get the Capability record that fits the role name given.
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Consequence<Option<Self>> {
        table
            .filter(dsl::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    /* ------------------------------- NOT SO STATIC ------------------------------ */

    /// Get the capability linked to a role
    pub fn capabilities(&self, conn: &MysqlConnection) -> Consequence<Vec<CapabilityEntity>> {
        Ok(RelRoleCapabilityEntity::get_capabilities_for_role(
            &conn, &self,
        )?)
    }

    /// Clear the database stored capabilities linked to this role
    pub fn clear_capabilities(&self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(rc_table.filter(rc_dsl::role_id.eq(self.id))).execute(conn)?;
        Ok(())
    }

    /// Return all the roles with the corresponding capability as an array
    /// of `RoleCapabilities`
    pub fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(RoleEntity::all(conn)?
            .iter()
            .map(|r| Self::by_role(conn, &r))
            .collect::<Vec<Self>>())
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn by_role_name(conn: &MysqlConnection, name: &str) -> Consequence<Option<Self>> {
        Ok(RoleEntity::by_name(conn, name)?.map(|r| Self::by_role(conn, &r)))
    }

    /// Constructor of `RoleCapabilities` based on a `role::Role` object
    pub fn by_role(conn: &MysqlConnection, r: &RoleEntity) -> Self {
        Self {
            id: r.id,
            name: r.name.to_string(),
            color: r.color.to_string(),
            //capabilities: RelRoleCapability::get_capabilities_for_role(&conn, &r),
        }
    }

    pub fn add_capabilities(
        &self,
        conn: &MysqlConnection,
        capabilities: &[CapabilityData],
    ) -> Consequence<()> {
        // for this new role, add every given capability
        for capability_data in capabilities.iter() {
            if let Some(capability) = CapabilityEntity::by_name(&*conn, &capability_data.name)? {
                RelRoleCapabilityEntity::add_capability_for_role(&*conn, &self, &capability)?;
            } else {
                // TODO : front-end sent an unexisting capability
            }
        }

        Ok(())
    }
}
