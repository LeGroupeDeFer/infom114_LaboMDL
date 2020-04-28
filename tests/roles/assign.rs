//! # Assign roles to users
//!
//! Here are grouped the tests that are meant to create role related data.
//!

/************************* REQUIRE *******************************************/

use rocket::http::ContentType;
use rocket::http::Status;

use super::super::init;
use unanimitylibrary::database::models::prelude::*;


const ROLE_ROUTE: &'static str = "/api/v1/user/role/";


/**************************** TESTS ******************************************/

#[test]
fn assign_role_to_user() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    // get a user
    let (user, _password) = init::get_user(true);

    // get a role
    let role_minima = RoleMinima {
        name: "newrole".to_string(),
        color: "#f0f0f0".to_string(),
    };
    let role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error")
    };

    let data = format!("{{ \"user_id\": {}, \"role_id\" : {} }}", user.id, role.id);

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert the user is now assigned to the role
    assert!(RelUserRoleEntity::get(&conn, user.id, role.id).unwrap().is_some());
}

#[test]
fn assign_role_to_user_without_requested_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (performing_user, password) = init::get_user(true);
    let auth_token_header = init::login(&performing_user.email, &password);

    // get a user
    let (user, _password) = init::get_user(true);

    // get a role
    let role_minima = RoleMinima {
        name: "newrole".to_string(),
        color: "#f0f0f0".to_string(),
    };

    let role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error")
    };

    let data = format!("{{ \"user_id\": {}, \"role_id\" : {} }}", user.id, role.id);

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Forbidden);

    // assert the user do not have the role

    assert!(RelUserRoleEntity::get(&conn, user.id, role.id).unwrap().is_none());
}

#[test]
fn assign_role_to_user_missing_user_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    // get a user
    let (user, _password) = init::get_user(true);

    // get a role
    let role_minima = RoleMinima {
        name: "newrole".to_string(),
        color: "#f0f0f0".to_string(),
    };
    let role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error")
    };

    let data = format!("{{ \"role_id\" : {} }}", role.id);

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // assert the user is now assigned to the role
    assert!(RelUserRoleEntity::get(&conn, user.id, role.id).unwrap().is_none());
}

#[test]
fn assign_role_to_user_missing_role_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    // get a user
    let (user, _password) = init::get_user(true);

    // get a role
    let role_minima = RoleMinima {
        name: "newrole".to_string(),
        color: "#f0f0f0".to_string(),
    };
    let role = match RoleEntity::insert_new(&conn, &role_minima) {
        Err(Error::EntityError(EntityError::Duplicate)) => panic!("The role already existed"),
        Ok(r) => r,
        _ => panic!("Internal error")
    };

    let data = format!("{{ \"user_id\": {} }}", user.id);

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // assert the user is now assigned to the role
    assert!(RelUserRoleEntity::get(&conn, user.id, role.id).unwrap().is_none());
}
