//! # Auth Tests
//!
//! Here should be grouped every test that is related to the
//! user's authentication.

/************************* REQUIRE *******************************************/
mod init;

use diesel::result::Error;
use diesel::Connection;

use rocket::http::{ContentType, Status};

use unanimitylibrary::authentication::forms::RegisterCredentials;
use unanimitylibrary::database::connection;
use unanimitylibrary::models::quick_response::Info;

/**************************** TESTS ******************************************/

#[test]
fn test_valid_mail() {
    init::clean();
    let client = init::client();

    let req = client
        .post("/api/register/check_email")
        .header(ContentType::JSON)
        .body("{\"email\": \"guillaume.latour@student.unamur.be\"}");

    let mut response = req.dispatch();
    let json_response: Info = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert!(json_response.success());
}

#[test]
fn register_new_user() {
    init::clean();
    let client = init::client();

    let test_user = RegisterCredentials {
        email: String::from("guillaume.latour@student.unamur.be"),
        password: String::from("mysuperpassword"),
        firstname: String::from("Guillaume"),
        lastname: String::from("Latour"),
        street: Some(String::from("my street")),
        number: Some(42),
        city: Some(String::from("Namur")),
        zipcode: Some(5000),
        country: Some(String::from("Belgium")),
        phone: None,
    };
    let req = client
        .post("/api/register/")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&test_user).unwrap());

    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);

    // TODO : check that the user is correctly added in the database
}
