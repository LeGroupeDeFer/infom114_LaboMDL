//! # Activation
//!
//! Here are grouped the registration tests
//! Those tests attack the `/api/auth/activate` route.

/************************* REQUIRE *******************************************/

use rocket::http::{ContentType, Status};

use unanimitylibrary::database::models::prelude::UserEntity;

use super::super::init;

const ROUTE: &'static str = "/api/v1/auth/activate";

/**************************** TESTS ******************************************/

#[test]
fn activation_good_id_good_token() {
    let client = init::clean_client();
    let (user, _passwd) = init::get_user(false);
    let connection = init::database_connection();

    // assert the user is inactive
    assert!(!user.active);

    let data = format!(
        "{{\"id\":{}, \"token\":\"{}\"}}",
        user.id,
        user.activation_token(&connection).unwrap().unwrap()
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);

    let activated_user = UserEntity::by_email(&connection, &user.email).unwrap().unwrap();
    let consumed_token = activated_user.activation_token(&connection).unwrap().unwrap();

    assert!(activated_user.active);
    assert!(!consumed_token.valid());
}

#[test]
fn activation_wrong_id_good_token() {
    let client = init::clean_client();
    let (user, _passwd) = init::get_user(false);
    let connection = init::database_connection();

    // assert the user is inactive
    assert!(!user.active);

    let mut fake_id = 12;
    while fake_id == user.id {
        fake_id += 1;
    }

    let data = format!(
        "{{\"id\":{}, \"token\":\"{}\"}}",
        fake_id,
        user.activation_token(&connection).unwrap().unwrap()
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);

    let not_so_activated_user = UserEntity::by_email(&connection, &user.email).unwrap().unwrap();
    let not_so_consumed_token = not_so_activated_user.activation_token(&connection).unwrap().unwrap();

    assert!(!not_so_activated_user.active);
    assert!(!not_so_consumed_token.consumed);
}

#[test]
fn activation_good_id_wrong_token() {
    let client = init::clean_client();
    let (user, _passwd) = init::get_user(false);
    let connection = init::database_connection();

    // assert the user is inactive
    assert!(!user.active);

    let data = format!(
        "{{\"id\":{}, \"token\":\"{}\"}}",
        user.id, "thisisafaketoken"
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);

    let not_so_activated_user = UserEntity::by_email(&connection, &user.email).unwrap().unwrap();
    let not_so_consumed_token = not_so_activated_user.activation_token(&connection).unwrap().unwrap();

    assert!(!not_so_activated_user.active);
    assert!(!not_so_consumed_token.consumed);
}

#[test]
fn activation_wrong_id_wrong_token() {
    let client = init::clean_client();
    let (user, _passwd) = init::get_user(false);
    let connection = init::database_connection();

    // assert the user is inactive
    assert!(!user.active);

    let mut fake_id = 12;
    while fake_id == user.id {
        fake_id += 1;
    }

    let data = format!(
        "{{\"id\":{}, \"token\":\"{}\"}}",
        fake_id, "thisisafaketoken"
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(data);
    let response = request.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);

    let not_so_activated_user = UserEntity::by_email(&connection, &user.email).unwrap().unwrap();
    let not_so_consumed_token =  not_so_activated_user.activation_token(&connection).unwrap().unwrap();

    assert!(!not_so_activated_user.active);
    assert!(!not_so_consumed_token.consumed);
}

#[test]
fn double_activation() {
    let client = init::clean_client();
    let (user, _passwd) = init::get_user(false);
    let connection = init::database_connection();
    let token = user.activation_token(&connection).unwrap().unwrap();

    // assert the user is inactive
    assert!(!user.active);

    let data = format!(
        "{{\"id\":{}, \"token\":\"{}\"}}",
        user.id,
        token.hash
    );

    let request = client.post(ROUTE).header(ContentType::JSON).body(&data);
    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);


    let activated_user = UserEntity::by_email(&connection, &user.email).unwrap().unwrap();
    let consumed_token = activated_user.activation_token(&connection).unwrap().unwrap();

    assert!(activated_user.active);
    assert!(consumed_token.consumed);

    let request_bis = client.post(ROUTE).header(ContentType::JSON).body(&data);
    let response_bis = request_bis.dispatch();

    assert_eq!(response_bis.status(), Status::Unauthorized);

    // the request failed, but the user is still activated
    assert!(activated_user.active);
    assert!(consumed_token.consumed);
}
