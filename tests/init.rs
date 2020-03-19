use unanimitylibrary::database::{db_config, MyDbConn};
use unanimitylibrary::models::quick_response::Info;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

pub fn client() -> Client {
    let rocket = unanimitylibrary::rocket(db_config()).attach(MyDbConn::fairing());
    Client::new(rocket).expect("valid rocket instance")
}

#[test]
fn test_valid_mail() {
    let client = client();

    let req = client
        .post("/api/register/check_email")
        .header(ContentType::JSON)
        .body("{\"email\": \"guillaume.latour@student.unamur.be\"}");

    let mut response = req.dispatch();
    let json_response: Info = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert!(json_response.success());
}
