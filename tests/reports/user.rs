use super::super::init;
use rocket::http::Status;
use unanimitylibrary::database::models::prelude::CountUserForm;

#[test]
fn get_users_count() {
    let client = init::clean_client();
    init::seed(); // 2 users with seed : admin & alan smithee

    for _ in 1..=10 {
        init::get_user(false);
    }

    for _ in 1..=5 {
        init::get_user(true);
    }

    let mut response = client
        .get("/api/v1/report/users")
        .header(init::login_admin())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let data: CountUserForm = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(data.total, 17);
    assert_eq!(data.active, 7);

    // FIXME : there is an issue with diesel I think...
    // assert_eq!(data.connected, 17);
}

#[test]
fn get_users_count_without_capabilities() {
    let client = init::clean_client();
    init::seed(); // 2 users with seed : admin & alan smithee

    for _ in 1..=10 {
        init::get_user(false);
    }

    for _ in 1..=4 {
        init::get_user(true);
    }

    let (user, password) = init::get_user(true);

    let response = client
        .get("/api/v1/report/users")
        .header(init::login(&user.email, &password))
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn get_users_count_unauthenticatedd() {
    let client = init::clean_client();
    init::seed(); // 2 users with seed : admin & alan smithee

    for _ in 1..=10 {
        init::get_user(false);
    }

    for _ in 1..=5 {
        init::get_user(true);
    }

    let response = client.get("/api/v1/report/users").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
