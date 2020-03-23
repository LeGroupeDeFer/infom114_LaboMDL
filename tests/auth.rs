//! # Auth Tests
//!
//! Here should be grouped every test that is related to the
//! user's authentication.

/************************* REQUIRE *******************************************/
mod init;

use diesel::query_dsl::RunQueryDsl;

use rocket::http::{ContentType, Status};

use unanimitylibrary::database::models::address::Address;
use unanimitylibrary::database::models::user::User;
use unanimitylibrary::database::schema::addresses::dsl::addresses;
use unanimitylibrary::database::schema::users::dsl::users;

/**************************** TESTS ******************************************/

#[test]
fn register_new_user() {
    let client = init::clean_client();
    let conn = init::database_connection();

    // check that there is no user in database already done in init::clean()

    // prepare a user
    let test_user = "{
        \"email\":\"guillaume.latour@student.unamur.be\",
        \"password\":\"mysuperpassword\",
        \"firstname\": \"Guillaume\",
        \"lastname\":\"Latour\"
     }";

    // request the application on the route /api/register
    let req = client
        .post("/api/auth/register/")
        .header(ContentType::JSON)
        .body(test_user);
    let response = req.dispatch();

    // check that the response is OK
    assert_eq!(response.status(), Status::Ok);

    // load users present in database
    let tab = users.load::<User>(&conn).unwrap();

    // check that there is only one user in database
    assert_eq!(tab.len(), 1);

    // check that this user is the one we just added
    assert_eq!(tab[0].email, "guillaume.latour@student.unamur.be");
    // and there is nothing in the address table
    assert_eq!(addresses.load::<Address>(&conn).unwrap().len(), 0);
}

#[test]
fn register_full_address() {
    let client = init::clean_client();
    let conn = init::database_connection();

    // check that there is no user in database already done in init::clean()

    // prepare a user
    let test_user = "{
        \"email\":\"guillaume.latour@student.unamur.be\",
        \"password\":\"mysuperpassword\",
        \"firstname\": \"Guillaume\",
        \"lastname\":\"Latour\",
        \"phone\":\"+32 471 85 85 85\",
        \"address\":{
            \"street\":\"rue grandganage\",
            \"number\":21,
            \"city\":\"Namur\",
            \"zipcode\":5000,
            \"country\":\"Belgique\"
        }
     }";

    // request the application on the route /api/register
    let req = client
        .post("/api/auth/register/")
        .header(ContentType::JSON)
        .body(test_user);
    let response = req.dispatch();

    // check that the response is OK
    assert_eq!(response.status(), Status::Ok);

    // load users present in database
    let tab_users = users.load::<User>(&conn).unwrap();

    // check that there is only one user in database
    assert_eq!(tab_users.len(), 1);

    // check that this user is the one we just added
    assert_eq!(tab_users[0].email, "guillaume.latour@student.unamur.be");

    // load address present in database
    let tab_address = addresses.load::<Address>(&conn).unwrap();

    // check there is only one address in database
    assert_eq!(tab_address.len(), 1);

    // check that this address is the one we just added
    assert_eq!(tab_address[0].street, "rue grandganage");
}

// test register avec addresse ok
// test register addresse pas ok
// test register existing email
// test double register same address

// test activation good code good id
// test activation wrong code good id
// test activation good code wrong id
// test activation wrong code wrong id
// test double activation

// test login inactive account
// test login active account
