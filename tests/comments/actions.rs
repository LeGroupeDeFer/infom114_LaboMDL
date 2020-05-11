use super::super::init;
use super::helper::*;
use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::comment::CommentEntity;
use unanimitylibrary::database::models::Entity;

#[test]
fn upvote_downvote_comment() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let (user1, password1) = init::get_user(true);
    let (user2, password2) = init::get_user(true);
    let (user3, password3) = init::get_user(true);
    let auth_token_header1 = init::login(&user1.email, &password1);
    let auth_token_header2 = init::login(&user2.email, &password2);
    let auth_token_header3 = init::login(&user3.email, &password3);

    let resp = send_vote(&client, auth_token_header1.clone(), &comment.id, 1);
    assert_eq!(resp.status(), Status::Ok);
    let tmp_comment = get_comment(&client, auth_token_header1.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 1);
    assert_eq!(tmp_comment.user_vote.unwrap(), 1);
    assert_eq!(tmp_comment.votes, 1);

    let resp = send_vote(&client, auth_token_header2.clone(), &comment.id, 1);
    assert_eq!(resp.status(), Status::Ok);
    let tmp_comment = get_comment(&client, auth_token_header2.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 2);
    assert_eq!(tmp_comment.user_vote.unwrap(), 1);
    assert_eq!(tmp_comment.votes, 2);

    let resp = send_vote(&client, auth_token_header3.clone(), &comment.id, -1);
    assert_eq!(resp.status(), Status::Ok);
    let tmp_comment = get_comment(&client, auth_token_header3.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 1);
    assert_eq!(tmp_comment.user_vote.unwrap(), -1);
    assert_eq!(tmp_comment.votes, 3);

    let resp = send_vote(&client, auth_token_header3.clone(), &comment.id, 2);
    assert_eq!(resp.status(), Status::BadRequest);
    let tmp_comment = get_comment(&client, auth_token_header3.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 1);
    assert_eq!(tmp_comment.user_vote.unwrap(), -1);
    assert_eq!(tmp_comment.votes, 3);

    let resp = send_vote(&client, auth_token_header2.clone(), &comment.id, 0);
    assert_eq!(resp.status(), Status::Ok);
    let tmp_comment = get_comment(&client, auth_token_header2.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 0);
    assert_eq!(tmp_comment.user_vote.unwrap(), 0);
    assert_eq!(tmp_comment.votes, 2);
}

#[test]
fn upvote_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .body(format!("{{ \"vote\":{} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Unauthorized);
}

#[test]
fn upvote_comment_malformed_json() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"vote\"{} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
}

#[test]
fn upvote_comment_missing_property() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"not_vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::UnprocessableEntity);
}

#[test]
fn upvote_comment_deleted() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, true);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
}

#[test]
fn upvote_comment_unexisting_comment_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let mut unexisting_id = 1;
    while CommentEntity::by_id(&conn, &unexisting_id)
        .unwrap()
        .is_some()
    {
        unexisting_id += 1;
    }

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &unexisting_id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
}

#[test]
fn upvote_comment_locked() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, true, false, false);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
}

#[test]
fn upvote_comment_locked_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, true, false, false);
    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(auth_token)
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Forbidden);
}

#[test]
fn upvote_comment_post_locked_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(true, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(auth_token)
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Forbidden);
}

#[test]
fn upvote_comment_hidden() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, true, false);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
}

#[test]
fn upvote_comment_hidden_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, true, false);
    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(auth_token)
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Forbidden);
}

#[test]
fn upvote_comment_post_hidden_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, true, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let (user, password) = init::get_user(true);
    let auth_token = init::login(&user.email, &password);

    let resp = client
        .post(format!("{}/{}/vote", COMMENT_ROUTE, &comment.id))
        .header(ContentType::JSON)
        .header(auth_token)
        .body(format!("{{ \"vote\": {} }}", 1))
        .dispatch();

    assert_eq!(resp.status(), Status::Forbidden);
}

#[test]
fn hide_unhide_comment() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment_entity = init::get_comment_entity(post.id, false, false, false);

    let comment = get_comment(&client, init::login_admin(), &comment_entity.id);
    assert!(!comment.hidden);

    let comment = toggle_comment_hide(&client, init::login_admin(), &comment_entity.id);
    assert!(comment.hidden);

    let comment = toggle_comment_hide(&client, init::login_admin(), &comment_entity.id);
    assert!(!comment.hidden);
}

#[test]
fn hide_invalid_comment_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let mut unexisting_id = 1;
    while CommentEntity::by_id(&conn, &unexisting_id)
        .unwrap()
        .is_some()
    {
        unexisting_id += 1;
    }

    let route = format!("{}/{}/hide", COMMENT_ROUTE, unexisting_id);
    let response = client.post(route).header(init::login_admin()).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn hide_comment_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment_entity = init::get_comment_entity(post.id, false, false, false);
    let (user, password) = init::get_user(true);

    let route = format!("{}/{}/hide", COMMENT_ROUTE, &comment_entity.id);
    let response = client
        .post(route)
        .header(init::login(&user.email, &password))
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn lock_unlock_comment() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment_entity = init::get_comment_entity(post.id, false, false, false);

    let comment = get_comment(&client, init::login_admin(), &comment_entity.id);
    assert!(!comment.locked);

    let comment = toggle_comment_lock(&client, init::login_admin(), &comment_entity.id);
    assert!(comment.locked);

    let comment = toggle_comment_lock(&client, init::login_admin(), &comment_entity.id);
    assert!(!comment.locked);
}

#[test]
fn lock_invalid_comment_id() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let mut unexisting_id = 1;
    while CommentEntity::by_id(&conn, &unexisting_id)
        .unwrap()
        .is_some()
    {
        unexisting_id += 1;
    }

    let route = format!("{}/{}/lock", COMMENT_ROUTE, unexisting_id);
    let response = client.post(route).header(init::login_admin()).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn lock_comment_missing_capability() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment_entity = init::get_comment_entity(post.id, false, false, false);
    let (user, password) = init::get_user(true);

    let route = format!("{}/{}/lock", COMMENT_ROUTE, &comment_entity.id);
    let response = client
        .post(route)
        .header(init::login(&user.email, &password))
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
}

// report a comment
// report a comment (invalid id)
// report a comment (invalid capability)
// report same comment twice
