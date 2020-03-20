//! # Test init
//!
//! Initialisations and helpers to ease the developpment of automated tests.
//!
//! the test database MUST be availlable
//! the migrations MUST have been applied to the test database

use unanimitylibrary::database::{self};
use unanimitylibrary::schema::users::dsl::users;

use diesel::query_dsl::RunQueryDsl;
use rocket::local::Client;

/// Truncate all the tables
pub fn clean() {
    // get connection
    let conn = database::connection();

    // truncate all tables
    diesel::delete(users).execute(&conn).unwrap();
}

/// Get a client that can be used to perform some HTTP actions on the
/// Rocket routes of the unanimity application
pub fn client() -> Client {
    // get Rocket instance
    let rocket = unanimitylibrary::rocket();

    // return new Client
    Client::new(rocket).expect("valid rocket instance")
}
