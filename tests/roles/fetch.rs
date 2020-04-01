//! # Fetching
//!
//! Here are grouped the tests that are meant to retreive some information.
//!
//! Both role & capability fetching are tested here.

/************************* REQUIRE *******************************************/

use rocket::http::{Header, Status};
// use rocket::http::{ContentType, Status};

use super::super::init;

const CAPABILITIES_ROUTE: &'static str = "/api/capabilities/";

/**************************** TESTS ******************************************/

#[test]
fn get_capabilities() {
    use unanimitylibrary::database::models::roles::capability::Capability;

    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let (user, passwd) = init::get_user(true);
    let auth_token = init::login(&user.email, &passwd);

    let request = client.get(CAPABILITIES_ROUTE).header(Header::new(
        "authorization",
        format!("{}{}", unanimitylibrary::auth::TOKEN_PREFIX, &auth_token),
    ));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();

    let request_capabilities: Vec<Capability> = serde_json::from_str(&data).unwrap();

    // assert those are the same capabilities as the one in database
    for c in Capability::all(&conn) {
        assert!(request_capabilities.contains(&c));
    }
}
