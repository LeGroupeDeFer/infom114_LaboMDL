//! # Delete roles
//!
//! Here are grouped the tests that are meant to delete role related data.
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
fn delete_correctly() {
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
    let auth_token_header = init::login_admin();

    // request
    let request = client
        .delete(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert the role has correctly been deleted
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_none());
}

#[test]
fn delete_missing_role_id() {
    // init
    let client = init::clean_client();
    init::seed();

    // login
    let auth_token_header = init::login_admin();

    // request
    let request = client
        .delete(format!("{}/", ROLE_ROUTE))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn delete_invalid_role_id() {
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
    let auth_token_header = init::login_admin();

    // request
    let request = client
        .delete(format!("{}/{}", ROLE_ROUTE, fake_id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn delete_missing_capability() {
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

    // request
    let request = client
        .delete(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Forbidden);

    // assert the role has not been deleted
    assert!(RoleEntity::by_name(&conn, &role_minima.name)
        .unwrap()
        .is_some());
}
