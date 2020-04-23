//! # RoleCapability module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `roles_capabilities` table

use crate::database::models::prelude::{CapabilityEntity, RoleEntity};
use crate::database::schema::{capabilities, roles_capabilities};
use crate::database::tables::{capabilities_table, roles_capabilities_table};
use crate::database::Data;

use diesel::prelude::*;
use diesel::MysqlConnection;

/// The struct `RelRoleCapability` is the exact representation of the
/// `role_capabilities` table.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "roles_capabilities"]
#[belongs_to(RoleEntity, foreign_key = "role_id")]
#[belongs_to(CapabilityEntity, foreign_key = "capability_id")]
pub struct RelRoleCapabilityEntity {
    pub id: u32,
    pub role_id: u32,
    pub capability_id: u32,
}

/// The struct `RelRoleCapabilityMinima` is used to insert some data inside the
/// `role_capabilities` table.
#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name = "roles_capabilities"]
pub struct RelRoleCapabilityMinima {
    pub role_id: u32,
    pub capability_id: u32,
}

impl RelRoleCapabilityEntity {
    /// Helper to get the capabilities of a role
    pub fn get_capabilities_for_role(
        conn: &MysqlConnection,
        role: &RoleEntity,
    ) -> Vec<CapabilityEntity> {
        let capabilities_id = Self::belonging_to(role).select(roles_capabilities::capability_id);

        capabilities_table
            .filter(capabilities::id.eq_any(capabilities_id))
            .load::<CapabilityEntity>(conn)
            .expect("problem fetching capabilities from role")
    }

    /// Constructor of `RelRoleCapability` based on a role id and a capability id
    pub fn get(conn: &MysqlConnection, role_id: u32, capability_id: u32) -> Option<Self> {
        roles_capabilities_table
            .filter(
                roles_capabilities::role_id
                    .eq(role_id)
                    .and(roles_capabilities::capability_id.eq(capability_id)),
            )
            .first(conn)
            .ok()
    }

    /// Insert a new row inside the `roles_capabilities` table.
    pub fn add_capability_for_role(
        conn: &MysqlConnection,
        role: &RoleEntity,
        capability: &CapabilityEntity,
    ) -> Data<Self> {
        match Self::get(&conn, role.id, capability.id) {
            Some(e) => Data::Existing(e),
            None => {
                diesel::insert_into(roles_capabilities_table)
                    .values(&RelRoleCapabilityMinima {
                        role_id: role.id,
                        capability_id: capability.id,
                    })
                    .execute(conn)
                    .expect("error while inserting rel role capability");
                Data::Inserted(
                    Self::get(&conn, role.id, capability.id)
                        .expect("Insert succesfful but error while fetching it"),
                )
            }
        }
    }
}
