use crate::database::models::roles::{capability, role, role_capability::RelRoleCapability};
use crate::database::Data;
use diesel::MysqlConnection;

pub fn seed_roles_and_capabilities(conn: &MysqlConnection) {
    let admin_minima = role::RoleMinima {
        name: "admin".to_string(),
        color: "#6bbaec".to_string(),
    };
    let user_minima = role::RoleMinima {
        name: "user".to_string(),
        color: "#8fd5a6".to_string(),
    };

    let admin = match role::Role::insert_minima(&conn, &admin_minima) {
        Data::Existing(a) => a,
        Data::Inserted(a) => a,
    };

    role::Role::insert_minima(&conn, &user_minima);

    for capability_minima in capability::CAPABILITIES
        .iter()
        .map(|capa| capability::CapabilityMinima {
            name: capa.to_string(),
        })
        .collect::<Vec<capability::CapabilityMinima>>()
        .iter()
    {
        let capa = match capability::Capability::insert_minima(&conn, &capability_minima) {
            Data::Existing(c) => c,
            Data::Inserted(c) => c,
        };
        RelRoleCapability::add_capability_for_role(&conn, &admin, &capa);
    }
}
