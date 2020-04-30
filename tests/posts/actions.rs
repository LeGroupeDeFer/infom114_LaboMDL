use rocket::http::Status;

use super::super::init;

use rocket::http::ContentType;
use unanimitylibrary::database::models::prelude::*;

const POST_ROUTE: &'static str = "/api/v1/post";

fn json_vote(value: i8) -> String {
    format!("{{ \"vote\":{} }}", value)
}

fn send_vote<'a, 'b>(
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

// upvote a post without being logged in
// upvote a post with malformed json
// upvote a soft-deleted post
// upvote a post (wrong id)
// upvote a locked post
// upvote an hidden post

// hide and unhide a post
// hide a post (invalid id)
// hide a post (invalid capability)

// lock and unlock a post
// lock a post (invalid id)
// lock a post (invalid capability)

// report a post
// report a post (invalid id)
// report a post (invalid capability)
// report same post twice
