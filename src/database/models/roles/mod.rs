pub mod capability;
pub mod forms;
pub mod role;
pub mod role_capability;

use crate::database::models::roles::capability::Capability;

use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RoleCapabilities {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub capabilities: Vec<Capability>,
}

impl RoleCapabilities {
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        let roles = role::Role::all(conn);

        roles
            .iter()
            .map(|r| RoleCapabilities {
                id: r.id,
                name: r.name.to_string(),
                color: r.color.to_string(),
                capabilities: role_capability::RelRoleCapability::get_capabilities_for_role(
                    &conn, &r,
                ),
            })
            .collect::<Vec<RoleCapabilities>>()
    }
}
