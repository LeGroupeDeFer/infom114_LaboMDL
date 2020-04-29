//! # Update roles
//!
//! Here are grouped the tests that are meant to update role related data.
//!

/************************* REQUIRE *******************************************/

use rocket::http::ContentType;
use rocket::http::Status;

use super::super::init;
use unanimitylibrary::database::models::prelude::*;
use unanimitylibrary::lib::{EntityError, Error};

const ROLE_ROUTE: &'static str = "/api/v1/role/";

/**************************** TESTS ******************************************/

#[test]
fn update_everything() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    // assert the role is not already in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_none());

    // insert the role
    let existing_role = RoleEntity::insert_new(&conn, &role_minima).unwrap();

    // assert the role is correctly added in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert there is a role with this new name in database
    let role_option = RoleEntity::by_name(&conn, role_name).unwrap();
    assert!(role_option.is_some());
    let role = role_option.unwrap();

    assert_eq!(role_name, role.name);
    assert_eq!(role_color, role.color);
    // if it panics, the test cannot pass !
    let role_capa = Role::by_role_name(&conn, &role.name).unwrap().unwrap();

    assert_eq!(role_capa.capabilities.len(), role_capabilities.len());
    for capability in role_capa.capabilities {
        assert!(role_capabilities.contains(&&capability.name[..]));
    }
}

#[test]
fn update_same_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error"),
    };
    // assert the role exists
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists (since its the same name)
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_some());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert there is a role with this new name in database
    let role_option = RoleEntity::by_name(&conn, role_name).unwrap();
    assert!(role_option.is_some());
    let role = role_option.unwrap();

    assert_eq!(role_name, role.name);
    assert_eq!(role_color, role.color);
    // if it panics, the test cannot pass !
    let role_capa = Role::by_role_name(&conn, &role.name).unwrap().unwrap();

    assert_eq!(role_capa.capabilities.len(), role_capabilities.len());
    for capability in role_capa.capabilities {
        assert!(role_capabilities.contains(&&capability.name[..]));
    }
}

#[test]
fn update_missing_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/", ROLE_ROUTE))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn update_invalid_role_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll find an unexisting role id
    let mut fake_id = 11;
    while let Some(_) = RoleEntity::by_id(&conn, &fake_id).unwrap() {
        fake_id += 11;
    }

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, fake_id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn update_no_color() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error"),
    };
    // assert the role is correctly added in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_missing_role_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error"),
    };
    // assert the role is correctly added in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_missing_role_capabilities() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error"),
    };
    // assert the role is correctly added in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\"
    }}",
        role_name, role_color
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_without_correct_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error"),
    };
    // assert the role is correctly added in database
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["user:manage_role", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(RoleEntity::by_name(&conn, role_name).unwrap().is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Forbidden);
}
