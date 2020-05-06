use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::prelude::*;

pub const POSTS_ROUTE: &'static str = "/api/v1/posts";
pub const POST_ROUTE: &'static str = "/api/v1/post";

pub fn get_posts_limit_and_offset(
    client: &rocket::local::Client,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Vec<Post> {
    let route: String = if limit.is_some() && offset.is_some() {
        format!(
            "{}?limit={}&offset={}",
            POSTS_ROUTE,
            limit.unwrap(),
            offset.unwrap()
        )
    } else if let Some(l) = limit {
        format!("{}?limit={}", POSTS_ROUTE, l)
    } else if let Some(o) = offset {
        format!("{}?offset={}", POSTS_ROUTE, o)
    } else {
        POSTS_ROUTE.to_string()
    };

    let req = client.get(route);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn send_create_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_title: &str,
    post_content: &str,
    post_kind: &str,
    tags: &[&str],
) -> Post {
    let json_post = format!(
        "{{ \
    \"title\": \"{}\",\
    \"content\": \"{}\",\
    \"kind\" : \"{}\",\
    \"tags\" : [{}]
    }}",
        post_title,
        post_content,
        post_kind,
        tags.iter()
            .map(|&t| format!("\"{}\"", t))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let mut response = client
        .post(POST_ROUTE)
        .header(auth_token)
        .header(ContentType::JSON)
        .body(json_post)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn json_vote(value: i8) -> String {
    format!("{{ \"vote\":{} }}", value)
}

pub fn send_vote<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
    vote_value: i8,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/vote", POST_ROUTE, post_id);
    client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(json_vote(vote_value))
        .dispatch()
}

pub fn toggle_visibility<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/hide", POST_ROUTE, post_id);
    client.post(route).header(auth_token).dispatch()
}

pub fn toggle_lock<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/lock", POST_ROUTE, post_id);
    client.post(route).header(auth_token).dispatch()
}

pub fn toggle_report<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
    reason: Option<&str>,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/report", POST_ROUTE, post_id);
    match reason {
        Some(r) => {
            let reason_json = format!("{{ \"reason\": \"{}\" }}", r);
            client
                .post(route)
                .header(ContentType::JSON)
                .header(auth_token)
                .body(reason_json)
                .dispatch()
        }
        None => client
            .post(route)
            .header(ContentType::JSON)
            .header(auth_token)
            .dispatch(),
    }
}

pub fn get_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
) -> Post {
    let route = format!("{}/{}", POST_ROUTE, post_id);
    let mut response = client.get(&route).header(auth_token).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn create_a_poll_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    title: &str,
    propositions: &[&str],
) -> Post {
    let json_data = format!(
        "{{\
        \"title\": \"{}\",\
        \"content\": \"\",\
        \"kind\": \"poll\",\
        \"options\": [{}]
    }}",
        title,
        propositions
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let mut response = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token)
        .body(json_data)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn get_poll_info(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
) -> PostPoll {
    let mut response = client
        .get(format!("{}/{}/poll", POST_ROUTE, post_id))
        .header(auth_token)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn send_poll_vote(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &u32,
    answer_id: &u32,
) -> PostPoll {
    let json_data = format!("{{ \"answer_id\": {} }}", answer_id);
    let mut response = client
        .post(format!("{}/{}/poll", POST_ROUTE, post_id))
        .header(auth_token)
        .header(ContentType::JSON)
        .body(json_data)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}
