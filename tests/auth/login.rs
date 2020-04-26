//! # Login
//!
//! Here are grouped the login tests
//! Those tests attack the `/api/auth/login` route.

/************************* REQUIRE *******************************************/

use rocket::http::{ContentType, Status};

use super::super::init;

const ROUTE: &'static str = "/api/v1/auth/login/";

/**************************** TESTS ******************************************/

#[test]
fn login_user_activated_correct_credentials() {
    let client = init::clean_client();
    let (user, password) = init::get_user(true);

    let data = format!(
        "{{\"email\":\"{}\", \"password\":\"{}\"}}",
        user.email, password
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn login_user_activated_wrong_credentials() {
    let client = init::clean_client();
    let (user, _password) = init::get_user(true);

    let data = format!(
        "{{\"email\":\"{}\", \"password\":\"{}\"}}",
        user.email, "thisisnotacorrectpassword"
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn login_user_not_activated_correct_credentials() {
    let client = init::clean_client();
    let (user, password) = init::get_user(false);

    let data = format!(
        "{{\"email\":\"{}\", \"password\":\"{}\"}}",
        user.email, password
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn login_user_not_activated_wrong_credentials() {
    let client = init::clean_client();
    let (user, _password) = init::get_user(false);

    let data = format!(
        "{{\"email\":\"{}\", \"password\":\"{}\"}}",
        user.email, "thisisnotacorrectpassword"
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}
