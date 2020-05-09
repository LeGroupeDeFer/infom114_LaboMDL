use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::prelude::Comment;

pub fn send_comment_from_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
    comment: &str,
) -> Comment {
    let route = format!("/api/v1/post/{}/comment", post_id);
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
    let route = format!("/api/v1/post/{}/comment", post_id);
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
    let route = format!("/api/v1/comment/{}", comment_id);
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

pub fn send_comment_from_unavailable_comment(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &u32,
    reply: &str,
) -> Status {
    let route = format!("/api/v1/comment/{}", comment_id);
    let data_json = format!("{{ \"content\": \"{}\" }}", reply);
    let req = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(data_json);
    
    // I prefer returning a Response object but I don't know how
    req.dispatch().status()
}


pub fn get_comment(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    comment_id: &u32,
) -> Comment {
    let route = format!("/api/v1/comment/{}", comment_id);
    let mut response = client
        .get(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    serde_json::from_str(&body).unwrap()
}
