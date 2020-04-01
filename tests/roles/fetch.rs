//! # Fetching
//!
//! Here are grouped the tests that are meant to retreive some information.
//!
//! Both role & capability fetching are tested here.

/************************* REQUIRE *******************************************/

use rocket::http::{ContentType, Status};

use unanimitylibrary::database::models::user::User;

use super::super::init;

const CAPABILITIES_ROUTE: &'static str = "/api/capabilities/";

/**************************** TESTS ******************************************/

#[test]
fn get_capabilities() {
    let client = init::clean_client();
    init::seed();
    // let (user, _passwd) = init::get_user(true);
    let connection = init::database_connection();

    let request = client.get(CAPABILITIES_ROUTE);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.data();
    println!("{}", data);
    panic!("bite");
}
