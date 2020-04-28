use diesel::MysqlConnection;

use crate::database::models::Entity;
use crate::database::models::prelude::*;


pub fn seed_roles_and_capabilities(conn: &MysqlConnection) {
    // create some minima roles (admin & user)
    let admin_minima = RoleMinima {
        name: "admin".to_string(),
        color: "#6bbaec".to_string(),
    };
    let user_minima = RoleMinima {
        name: "user".to_string(),
        color: "#8fd5a6".to_string(),
    };

    // insert those roles in database
    RoleEntity::insert(&conn, &user_minima);
    let admin_role = RoleEntity::insert_either(&conn, &admin_minima).unwrap();

    // add every capability in database & link them to the admin role
    for capability_minima in CAPABILITIES
        .iter()
        .map(|cap| CapabilityMinima {
            name: cap.to_string(),
        })
        .collect::<Vec<CapabilityMinima>>()
        .iter()
    {
        let cap = CapabilityEntity::insert_either(&conn, &capability_minima).unwrap();
        RelRoleCapabilityEntity::add_capability_for_role(&conn, &admin_role, &cap);
    }

    // create the admin user
    let mut admin_user = UserEntity::insert_either(
        &conn,
        &UserMinima {
            email: "admin@unamur.be".to_string(),
            password: "admin".to_string(),
            firstname: "john".to_string(),
            lastname: "doe".to_string(),
            address: None,
            phone: None,
            activation_token: Some(TokenEntity::create_default(&conn).unwrap().id),
            refresh_token: Some(TokenEntity::create_default(&conn).unwrap().id),
            recovery_token: Some(TokenEntity::create_default(&conn).unwrap().id),
        },
    ).unwrap();

    // activate the admin
    admin_user.activate(&conn);

    // assign the role admin to the admin user
    RelUserRoleEntity::add_role_for_user(&conn, &admin_user, &admin_role);
}
