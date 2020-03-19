mod init;

use diesel::result::Error;
use diesel::Connection;
use rocket::http::{ContentType, Status};
use unanimitylibrary::authentication::forms::RegisterCredentials;
use unanimitylibrary::database::connection;

#[test]
fn register_new_user() {
    let client = init::client();
    let connexion = connection();

    connexion.test_transaction::<_, Error, _>(|| {
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
        Ok(())
    });

    assert_eq!(2 + 2, 4);
}
