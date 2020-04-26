use crate::database::models::Entity;
use crate::database::models::roles::{
    capability, role, role_capability::RelRoleCapability, user_role::RelUserRole,
};
use crate::database::models::user;
use crate::database::models::token;
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
    role::Role::insert(&conn, &user_minima);
    let admin_role = role::Role::insert_either(&conn, &admin_minima).unwrap();

    // add every capabilities in database & link them to the admin role
    for capability_minima in capability::CAPABILITIES
        .iter()
        .map(|capa| capability::CapabilityMinima {
            name: capa.to_string(),
        })
        .collect::<Vec<capability::CapabilityMinima>>()
        .iter()
    {
        let capa = capability::Capability::insert_either(&conn, &capability_minima).unwrap();
        RelRoleCapability::add_capability_for_role(&conn, &admin_role, &capa);
    }

    // create the admin user
    let mut admin_user = user::User::insert_either(
        &conn,
        &user::UserMinima {
            email: "admin@unamur.be".to_string(),
            password: "admin".to_string(),
            firstname: "john".to_string(),
            lastname: "doe".to_string(),
            address: None,
            phone: None,
            activation_token: Some(token::Token::create_default(&conn).unwrap().id),
            refresh_token: Some(token::Token::create_default(&conn).unwrap().id),
            recovery_token: Some(token::Token::create_default(&conn).unwrap().id),
        },
    ).unwrap();

    // activate the admin
    admin_user.activate(&conn);

    // assign the role admin to the admin user
    RelUserRole::add_role_for_user(&conn, &admin_user, &admin_role);
}
