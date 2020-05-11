use super::super::init;
use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::prelude::Comment;

pub const COMMENT_ROUTE: &'static str = "/api/v1/comment";
pub const POST_ROUTE: &'static str = "/api/v1/post";

pub fn send_comment_from_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
    comment: &str,
) -> Comment {
    let route = format!("{}/{}/comment", POST_ROUTE, post_id);
    let data_json = format!("{{ \"content\": \"{}\" }}", comment);

    let mut response = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(data_json)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn send_comment_from_unavailable_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
    comment: &str,
) -> Status {
    let route = format!("{}/{}/comment", POST_ROUTE, post_id);
    let data_json = format!("{{ \"content\": \"{}\" }}", comment);
    let req = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(data_json);

    // I prefer returning a Response object but I don't know how
    req.dispatch().status()
}

pub fn send_comment_from_comment(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &u32,
    reply: &str,
) -> Comment {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let data_json = format!("{{ \"content\": \"{}\" }}", reply);
    let mut response = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(data_json)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn send_comment_from_comment_ko<'a>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &u32,
    reply: &str,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let data_json = format!("{{ \"content\": \"{}\" }}", reply);
    let req = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(data_json);

    req.dispatch()
}

pub fn get_comment(
    client: &rocket::local::Client,
    auth_token_header: rocket::http::Header<'static>,
    comment_id: &u32,
) -> Comment {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);

    let mut response = client.get(route).header(auth_token_header).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn get_comment_normal_user_ok(client: &rocket::local::Client, comment_id: &u32) -> Comment {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let mut response = client.get(route).header(auth_token_header).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn get_comment_normal_user_ko(client: &rocket::local::Client, comment_id: &u32) -> Status {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let response = client.get(route).header(auth_token_header).dispatch();

    response.status()
}

pub fn get_comment_unauth_ok(client: &rocket::local::Client, comment_id: &u32) -> Comment {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let mut response = client.get(route).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn get_comment_unauth_ko(client: &rocket::local::Client, comment_id: &u32) -> Status {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let response = client.get(route).dispatch();

    response.status()
}

pub fn get_comment_admin_ok(client: &rocket::local::Client, comment_id: &u32) -> Comment {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let mut response = client.get(route).header(init::login_admin()).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}

pub fn get_comment_admin_ko(client: &rocket::local::Client, comment_id: &u32) -> Status {
    let route = format!("{}/{}", COMMENT_ROUTE, comment_id);
    let response = client.get(route).header(init::login_admin()).dispatch();

    response.status()
}

pub fn send_vote<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &'b u32,
    vote_value: i8,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/vote", COMMENT_ROUTE, comment_id);
    client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(format!("{{ \"vote\":{} }}", vote_value))
        .dispatch()
}

pub fn toggle_comment_hide(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &u32,
) -> Comment {
    let route = format!("{}/{}/hide", COMMENT_ROUTE, comment_id);

    let mut response = client.post(route).header(auth_token).dispatch();
    assert_eq!(response.status(), Status::Ok);

    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}
