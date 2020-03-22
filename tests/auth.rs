//! # Auth Tests
//!
//! Here should be grouped every test that is related to the
//! user's authentication.

/************************* REQUIRE *******************************************/
mod init;

use diesel::query_dsl::RunQueryDsl;

use rocket::http::{ContentType, Status};

use unanimitylibrary::authentication::forms::RegisterCredentials;
use unanimitylibrary::http::helpers::quick_response::Info;

use unanimitylibrary::database;
use unanimitylibrary::database::models::user::User;
use unanimitylibrary::schema::users::dsl::users;

/**************************** TESTS ******************************************/

#[test]
fn test_valid_mail() {
    let client = init::clean_client();

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
    let client = init::clean_client();

    // prepare a user
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

    // request the application on the route /api/register
    let req = client
        .post("/api/register/")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&test_user).unwrap());
    let response = req.dispatch();

    // check that the response is OK
    assert_eq!(response.status(), Status::Ok);

    // load users present in database
    let conn = database::connection();
    let tab = users.load::<User>(&conn).unwrap();

    // check that there is only one user in database
    assert_eq!(tab.len(), 1);

    // check that this user is the one we just added
    assert_eq!(tab[0].email, "guillaume.latour@student.unamur.be");
}
