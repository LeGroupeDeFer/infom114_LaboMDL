use crate::database::models::capabilities::CAPABILITIES;
use crate::database::models::prelude::{
    CapabilityEntity, CapabilityMinima, RelRoleCapabilityEntity, RelUserRoleEntity, RoleEntity,
    RoleMinima, UserEntity, UserMinima,
};
use crate::database::Data;
use diesel::MysqlConnection;

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
    RoleEntity::insert_minima(&conn, &user_minima);
    let admin_role = match RoleEntity::insert_minima(&conn, &admin_minima) {
        Data::Existing(a) => a,
        Data::Inserted(a) => a,
        _ => panic!("unreachable code reached"),
    };

    // add every capabilities in database & link them to the admin role
    for capability_minima in CAPABILITIES
        .iter()
        .map(|capa| CapabilityMinima {
            name: capa.to_string(),
        })
        .collect::<Vec<CapabilityMinima>>()
        .iter()
    {
        let capa = match CapabilityEntity::insert_minima(&conn, &capability_minima) {
            Data::Existing(c) => c,
            Data::Inserted(c) => c,
            _ => panic!("unreachable code reached"),
        };
        RelRoleCapabilityEntity::add_capability_for_role(&conn, &admin_role, &capa);
    }

    // create the admin user
    let admin_user = match UserEntity::insert_minima(
        &conn,
        &UserMinima {
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
        _ => panic!("unreachable code reached"),
    };

    // activate the admin
    admin_user.activate(&conn);

    // assign the role admin to the admin user
    RelUserRoleEntity::add_role_for_user(&conn, &admin_user, &admin_role);
}
