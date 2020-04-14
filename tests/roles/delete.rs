//! # Delete roles
//!
//! Here are grouped the tests that are meant to delete role related data.
//!

/************************* REQUIRE *******************************************/

use rocket::http::ContentType;
use rocket::http::Status;

use super::super::init;

const ROLE_ROUTE: &'static str = "/api/role/";

use unanimitylibrary::database::models::roles;
use unanimitylibrary::database::Data;

/**************************** TESTS ******************************************/

#[test]
fn delete_correctly() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    // request
    let request = client
        .delete(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert the role has correctly been deleted
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_none());
}

#[test]
fn delete_missing_role_id() {
    // init
    let client = init::clean_client();
    init::seed();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

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
    while let Some(_) = roles::role::Role::by_id(&conn, &fake_id) {
        fake_id += 11;
    }

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    // request
    let request = client
        .delete(format!("{}/{}", ROLE_ROUTE, fake_id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn delete_missing_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

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
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());
}
