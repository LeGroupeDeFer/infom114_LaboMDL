//! # Read roles
//!
//! Here are grouped the tests that are meant to retreive some information.
//!
//! Both role & capability fetching are tested here.

/************************* REQUIRE *******************************************/

use rocket::http::Status;
// use rocket::http::{ContentType, Status};

use super::super::init;

const CAPABILITIES_ROUTE: &'static str = "/api/capabilities/";
const ROLES_ROUTE: &'static str = "/api/roles/";

/**************************** TESTS ******************************************/

#[test]
fn get_capabilities() {
    use unanimitylibrary::database::models::roles::capability::Capability;

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

    // assert those are the same capabilities as the one in database
    let data = response.body_string().unwrap();
    let request_capabilities: Vec<Capability> = serde_json::from_str(&data).unwrap();
    for c in Capability::all(&conn) {
        assert!(request_capabilities.contains(&c));
    }
}

#[test]
fn get_roles() {
    use unanimitylibrary::database::models::roles::RoleCapabilities;

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
    let request_roles: Vec<RoleCapabilities> = serde_json::from_str(&data).unwrap();
    for c in RoleCapabilities::all(&conn) {
        assert!(request_roles.contains(&c));
    }
}
