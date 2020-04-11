use crate::database::models::roles::{
    capability, role, role_capability::RelRoleCapability, user_role::RelUserRole,
};
use crate::database::models::user;
use crate::database::Data;
use diesel::MysqlConnection;

pub fn seed_roles_and_capabilities(conn: &MysqlConnection) {
    // create some minima roles (admin & user)
    let admin_minima = role::RoleMinima {
        name: "admin".to_string(),
        color: "#6bbaec".to_string(),
    };
    let user_minima = role::RoleMinima {
        name: "user".to_string(),
        color: "#8fd5a6".to_string(),
    };

    // insert those roles in database
    role::Role::insert_minima(&conn, &user_minima);
    let admin_role = match role::Role::insert_minima(&conn, &admin_minima) {
        Data::Existing(a) => a,
        Data::Inserted(a) => a,
    };

    // add every capabilities in database & link them to the admin role
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
        RelRoleCapability::add_capability_for_role(&conn, &admin_role, &capa);
    }

    // create the admin user
    let admin_user = match user::User::insert_minima(
        &conn,
        &user::UserMinima {
            email: "admin@unamur.be".to_string(),
            password: "admin".to_string(),
            firstname: "john".to_string(),
            lastname: "doe".to_string(),
            address: None,
            phone: None,
        },
    ) {
        Data::Existing(a) => a,
        Data::Inserted(a) => a,
    };

    // activate the admin
    admin_user.activate(&conn);

    // assign the role admin to the admin user
    RelUserRole::add_role_for_user(&conn, &admin_user, &admin_role);
}
