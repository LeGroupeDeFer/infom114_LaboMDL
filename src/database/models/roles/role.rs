use crate::database;
use crate::database::models::prelude::{Capability, RelRoleCapabilityEntity, RoleEntity};
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
pub struct Role {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub capabilities: Vec<Capability>,
}

impl Role {
    /// Return all the roles with the corresponding capabilities as an array
    /// of `RoleCapabilities`
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        RoleEntity::all(conn)
            .drain(..)
            .map(|r| Self::from(r))
            .collect::<Vec<Self>>()
    }

    /// Constructor of `RoleCapabilities` based on a role name
    pub fn by_role_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        RoleEntity::by_name(conn, name).map(|r| Self::from(r))
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
                .drain(..)
                .map(|capability_entity| Capability::from(capability_entity))
                .collect::<Vec<Capability>>(),
        }
    }
}
