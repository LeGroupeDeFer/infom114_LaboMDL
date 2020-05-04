//! # Read roles
//!
//! Here are grouped the tests that are meant to retrieve some information.
//!
//! Both role & capability fetching are tested here.

/************************* REQUIRE *******************************************/

use rocket::http::Status;
// use rocket::http::{ContentType, Status};

use super::super::init;
use unanimitylibrary::database::models::prelude::*;

const CAPABILITIES_ROUTE: &'static str = "/api/v1/capability";
const ROLES_ROUTE: &'static str = "/api/v1/roles";

/**************************** TESTS ******************************************/

#[test]
fn get_capabilities() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let (user, passwd) = init::get_user(true);

    // login
    let auth_token_header = init::login(&user.email, &passwd);

    // request
    let request = client.get(CAPABILITIES_ROUTE).header(auth_token_header);
    let mut response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert those are the same capability as the one in database
    let data = response.body_string().unwrap();
    let request_capabilities: Vec<CapabilityEntity> = serde_json::from_str(&data).unwrap();
    for c in CapabilityEntity::all(&conn).unwrap() {
        assert!(request_capabilities.contains(&c));
    }
}

#[test]
fn get_roles() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let (user, passwd) = init::get_user(true);

    // login
    let auth_token_header = init::login(&user.email, &passwd);

    // build request
    let request = client.get(ROLES_ROUTE).header(auth_token_header);

    // get response
    let mut response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert those are the same roles as the one in database
    let data = response.body_string().unwrap();
    let request_roles: Vec<Role> = serde_json::from_str(&data).unwrap();
    for c in Role::all(&conn).unwrap() {
        assert!(request_roles.contains(&c));
    }
}
