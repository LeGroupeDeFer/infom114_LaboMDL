use rocket::http::Status;

use super::super::init;

use rocket::http::ContentType;
use unanimitylibrary::database::models::prelude::*;

const POST_ROUTE: &'static str = "/api/v1/post";

fn json_vote(value: i8) -> String {
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

fn toggle_visibility<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/hide", POST_ROUTE, post_id);
    client.post(route).header(auth_token).dispatch()
}

fn toggle_lock<'a, 'b>(
    client: &'a rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_id: &'b u32,
) -> rocket::local::LocalResponse<'a> {
    let route = format!("{}/{}/lock", POST_ROUTE, post_id);
    client.post(route).header(auth_token).dispatch()
}

fn toggle_report<'a, 'b>(
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

fn get_post(
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

#[test]
fn upvote_post() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;

    let (user1, password) = init::get_user(true);
    let auth_token_user1 = init::login(&user1.email, &password);
    let (user2, password) = init::get_user(true);
    let auth_token_user2 = init::login(&user2.email, &password);
    let (user3, password) = init::get_user(true);
    let auth_token_user3 = init::login(&user3.email, &password);

    // user 1 down vote
    let r1 = send_vote(&client, auth_token_user1.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::Ok);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 1);

    // user 2 down vote
    let r2 = send_vote(&client, auth_token_user2.clone(), &post.id, -1);
    assert_eq!(r2.status(), Status::Ok);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, -2);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 2);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    assert_eq!(tmp_post.score, -2);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 2);

    // user 3 up vote
    let r3 = send_vote(&client, auth_token_user3.clone(), &post.id, 1);
    assert_eq!(r3.status(), Status::Ok);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 3);

    // user 1 changes his mind and up vote
    let r4 = send_vote(&client, auth_token_user1.clone(), &post.id, 1);
    assert_eq!(r4.status(), Status::Ok);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 3);

    // user 2 makes a mistake and upvote for a value of 2 (nothing changes)
    let r5 = send_vote(&client, auth_token_user2.clone(), &post.id, 2);
    assert_eq!(r5.status(), Status::BadRequest);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 3);
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 3);

    // user 2 makes it right remove his vote
    let r6 = send_vote(&client, auth_token_user2.clone(), &post.id, 0);
    assert_eq!(r6.status(), Status::Ok);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    assert_eq!(tmp_post.score, 2);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 2);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    assert_eq!(tmp_post.score, 2);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 2);
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    assert_eq!(tmp_post.score, 2);
    assert_eq!(tmp_post.user_vote.unwrap(), 1);
    assert_eq!(tmp_post.votes, 2);
}

#[test]
fn upvote_and_remove_vote() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;

    let (user1, password) = init::get_user(true);
    let auth_token_user1 = init::login(&user1.email, &password);

    let mut r1 = send_vote(&client, auth_token_user1.clone(), &post.id, 1);
    assert_eq!(r1.status(), Status::Ok);
    tmp_post = serde_json::from_str(r1.body_string().unwrap().as_str()).unwrap();
    assert_eq!(tmp_post.score, 1);
    assert_eq!(tmp_post.user_vote, Some(1));

    let mut r2 = send_vote(&client, auth_token_user1.clone(), &post.id, 0);
    assert_eq!(r2.status(), Status::Ok);
    tmp_post = serde_json::from_str(r2.body_string().unwrap().as_str()).unwrap();
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote, Some(0));
}

#[test]
fn upvote_post_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let tmp_post: Post;

    let route = format!("{}/{}/vote", POST_ROUTE, &post.id);
    let response = client
        .post(route)
        .header(ContentType::JSON)
        .body(json_vote(1))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    tmp_post = get_post(&client, init::login_admin(), &post.id);
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn upvote_post_invalid_json() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let tmp_post: Post;

    let route = format!("{}/{}/vote", POST_ROUTE, &post.id);
    let json_vote = format!("{{ \"vote:{}, }}", 1);
    let response = client
        .post(route)
        .header(init::login_admin())
        .header(ContentType::JSON)
        .body(json_vote)
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    tmp_post = get_post(&client, init::login_admin(), &post.id);
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn upvote_post_missing_json_attribute() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let tmp_post: Post;

    let route = format!("{}/{}/vote", POST_ROUTE, &post.id);
    let json_vote = format!("{{ \"wrong_vote_name\":{} }}", 1);
    let response = client
        .post(route)
        .header(init::login_admin())
        .header(ContentType::JSON)
        .body(json_vote)
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);
    tmp_post = get_post(&client, init::login_admin(), &post.id);
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn upvote_post_deleted_post() {
    let conn = init::database_connection();
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, true);
    let tmp_post: Post;

    let auth_token = init::login_admin();

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::BadRequest);

    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote, None);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn upvote_post_invalid_id() {
    let conn = init::database_connection();
    let client = init::clean_client();
    init::seed();
    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    let auth_token = init::login_admin();

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &unexisting_id, -1);
    assert_eq!(r1.status(), Status::BadRequest);
}

#[test]
fn upvote_post_locked_post_as_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(true, false, false);
    let tmp_post: Post;

    let auth_token = init::login_admin();

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::Ok);

    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 1);
}

#[test]
fn upvote_post_locked_post_as_user() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(true, false, false);
    let tmp_post: Post;

    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::Forbidden);

    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn upvote_post_hidden_post_as_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, true, false);
    let tmp_post: Post;

    let auth_token = init::login_admin();

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::Ok);

    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert_eq!(tmp_post.score, -1);
    assert_eq!(tmp_post.user_vote.unwrap(), -1);
    assert_eq!(tmp_post.votes, 1);
}

#[test]
fn upvote_post_hidden_post_as_user() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, true, false);
    let tmp_post: Post;

    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    // down vote
    let r1 = send_vote(&client, auth_token.clone(), &post.id, -1);
    assert_eq!(r1.status(), Status::Forbidden);

    tmp_post = get_post(&client, init::login_admin(), &post.id);
    assert_eq!(tmp_post.score, 0);
    assert_eq!(tmp_post.user_vote.unwrap(), 0);
    assert_eq!(tmp_post.votes, 0);
}

#[test]
fn hide_a_post() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    // assert post is not hidden
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.hidden);

    // hide post
    let r1 = toggle_visibility(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::Ok);

    // assert post is hidden
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(tmp_post.hidden);

    // unhide post
    let r2 = toggle_visibility(&client, auth_token.clone(), &post.id);
    assert_eq!(r2.status(), Status::Ok);

    // assert post is not hidden anymore
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.hidden);
}

#[test]
fn hide_a_post_invalid_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let auth_token = init::login_admin();
    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    // hide post
    let r1 = toggle_visibility(&client, auth_token.clone(), &unexisting_id);
    assert_eq!(r1.status(), Status::BadRequest);
}

#[test]
fn hide_a_post_deleted() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let post = init::get_post_entity(false, false, true);
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    // assert the post is not hidden
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    assert!(!tmp_post.hidden);

    // hide post
    let r1 = toggle_visibility(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::BadRequest);

    // assert post is still not hidden
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    assert!(!tmp_post.hidden);
}

#[test]
fn hide_a_post_without_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;

    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    // assert post is not hidden
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.hidden);

    // hide post
    let r1 = toggle_visibility(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::Forbidden);

    // assert post is not hidden
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.hidden);
}

#[test]
fn lock_a_post() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    // assert post is not locked
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.locked);

    // lock post
    let r1 = toggle_lock(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::Ok);

    // assert post is locked
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(tmp_post.locked);

    // unlock post
    let r2 = toggle_lock(&client, auth_token.clone(), &post.id);
    assert_eq!(r2.status(), Status::Ok);

    // assert post is not locked anymore
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.locked);
}

#[test]
fn lock_a_post_invalid_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let auth_token = init::login_admin();
    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    // lock post
    let r1 = toggle_lock(&client, auth_token.clone(), &unexisting_id);
    assert_eq!(r1.status(), Status::BadRequest);
}

#[test]
fn lock_a_post_deleted() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let post = init::get_post_entity(false, false, true);
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    // assert the post is not locked
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    assert!(!tmp_post.locked);
    assert!(tmp_post.deleted);

    // lock post
    let r1 = toggle_lock(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::BadRequest);

    // assert post is still not locked
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    assert!(!tmp_post.locked);
    assert!(tmp_post.deleted);
}

#[test]
fn lock_a_post_without_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let mut tmp_post: Post;

    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    // assert post is not locked
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.locked);

    // lock post
    let r1 = toggle_lock(&client, auth_token.clone(), &post.id);
    assert_eq!(r1.status(), Status::Forbidden);

    // assert post is not locked
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    assert!(!tmp_post.locked);
}

#[test]
fn report_a_post() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let conn = init::database_connection();
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    let (user1, password) = init::get_user(true);
    let auth_token_user1 = init::login(&user1.email, &password);
    let (user2, password) = init::get_user(true);
    let auth_token_user2 = init::login(&user2.email, &password);
    let (user3, password) = init::get_user(true);
    let auth_token_user3 = init::login(&user3.email, &password);

    // assert post is not reported
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 0);

    // report post by admin
    let r0 = toggle_report(&client, auth_token.clone(), &post.id, Some("random reason"));
    assert_eq!(r0.status(), Status::Ok);

    // assert post is reported by admin
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 1);
    // assert the post is not reported by user1
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user1.id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 1);

    // report post with user 1
    let r1 = toggle_report(
        &client,
        auth_token_user1.clone(),
        &post.id,
        Some("random reason"),
    );
    assert_eq!(r1.status(), Status::Ok);

    // assert post is reported by admin & user 1
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user1.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
    // assert post is not reported by user 3
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user3.id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);

    // report post with user 2
    let r2 = toggle_report(
        &client,
        auth_token_user2.clone(),
        &post.id,
        Some("random reason"),
    );
    assert_eq!(r2.status(), Status::Ok);

    // assert admin, user 1 and user 2 do have a report
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user1.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user2.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    // assert post is not reported by user 3
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user3.id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);

    // user 3 try to remove his (unexisting) report
    let r3 = toggle_report(&client, auth_token_user3.clone(), &post.id, None);
    assert_eq!(r3.status(), Status::BadRequest);

    // assert admin, user 1 and user 2 do have a report
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user1.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user2.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);
    // assert post is not reported by user 3
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user3.id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 3);

    // remove a report post with admin
    let r4 = toggle_report(&client, auth_token.clone(), &post.id, None);
    assert_eq!(r4.status(), Status::Ok);

    // assert user 1 and user 2 do have a report
    tmp_post = get_post(&client, auth_token_user1.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user1.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
    tmp_post = get_post(&client, auth_token_user2.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user2.id);
    assert!(tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
    // assert post is not reported by admin neither user 3
    tmp_post = get_post(&client, auth_token.clone(), &post.id);
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
    tmp_post = get_post(&client, auth_token_user3.clone(), &post.id);
    tmp_post.set_user_info(&conn, &user3.id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 2);
}

#[test]
fn report_a_post_invalid_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let auth_token = init::login_admin();
    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    // report post
    let r1 = toggle_report(
        &client,
        auth_token.clone(),
        &unexisting_id,
        Some("random reason"),
    );
    assert_eq!(r1.status(), Status::BadRequest);
}

#[test]
fn report_a_post_deleted() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let post = init::get_post_entity(false, false, true);
    let mut tmp_post: Post;
    let auth_token = init::login_admin();

    // assert the post is not reported
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 0);
    assert!(tmp_post.deleted);

    // report post
    let r1 = toggle_report(&client, auth_token.clone(), &post.id, Some("random reason"));
    assert_eq!(r1.status(), Status::BadRequest);

    // assert post is still not reported
    tmp_post = Post::from(PostEntity::by_id(&conn, &post.id).unwrap().unwrap());
    tmp_post.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_post.user_flag.unwrap());
    assert_eq!(tmp_post.flags, 0);
    assert!(tmp_post.deleted);
}
