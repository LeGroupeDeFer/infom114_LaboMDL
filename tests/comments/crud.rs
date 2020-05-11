use std::{thread, time};

use super::super::init;
use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::prelude::*;

use super::helper::*;
use crate::init::login_admin;

#[test]
fn create_comment_from_post_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment_content = "FIIIIIRST!!!";
    let comment = send_comment_from_post(&client, login_admin(), &post.id, comment_content);

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.post_id, post.id);
    assert_eq!(comment_entity.parent_id, None);
    assert_eq!(comment_entity.content, comment_content);
    assert_eq!(comment_entity.author_id, comment.author.id);
    assert_eq!(comment_entity.author_id, init::get_admin().id);
}

#[test]
fn create_comment_from_post_normal_user() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    // init simple user
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, false, false);
    let comment_content = "Normal user";
    let comment = send_comment_from_post(
        &client,
        auth_token_header,
        &post.id,
        comment_content
    );

    let comment_entity = CommentEntity::by_id(&conn, &comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.post_id, post.id);
    assert_eq!(comment_entity.parent_id, None);
    assert_eq!(comment_entity.content, comment.content);
    assert_eq!(comment_entity.author_id, comment.author.id);
}

#[test]
fn create_duplicate_comments_from_post() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let token = login_admin();
    let comment_content = "There are 2 comments like this!!";

    let comment1 = send_comment_from_post(&client, token.clone(), &post.id, comment_content);
    assert!(CommentEntity::by_id(&conn, &comment1.id).unwrap().is_some());

    thread::sleep(time::Duration::from_millis(1000));

    let comment2 = send_comment_from_post(&client, token.clone(), &post.id, comment_content);
    assert!(CommentEntity::by_id(&conn, &comment2.id).unwrap().is_some());

    assert_ne!(comment1.id, comment2.id);
    assert!(comment1.created_at < comment2.created_at);
}

#[test]
fn create_comment_from_post_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let post = init::get_post_entity(false, false, false);
    let route = format!("/api/v1/post/{}/comment", post.id);

    let comment_content = "This is a test comment.";
    let json_data = format!("{{ \"content\": \"{}\" }}", comment_content);

    let resp = client
        .post(route)
        .header(ContentType::JSON)
        .body(json_data)
        .dispatch();

    assert_eq!(resp.status(), Status::Unauthorized);
    assert_eq!(
        CommentEntity::by_post_id(&conn, &post.id, false)
            .unwrap()
            .len(),
        0
    );
}

// create a comment with a malformed json
#[test]
fn create_comment_from_post_bad_json() {
    let client = init::clean_client();
    init::seed();

    let conn = init::database_connection();
    let auth_header = init::login_admin();
    let post = init::get_post_entity(false, false, false);
    let route = format!("/api/v1/post/{}/comment", post.id);

    let comment_content = "There is no quotation marks surrouding me.";
    let bad_json = format!("{{ \"content\": {} }}", comment_content);

    let resp = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_header)
        .body(bad_json)
        .dispatch();
    assert_eq!(resp.status(), Status::BadRequest);
    assert_eq!(
        CommentEntity::by_post_id(&conn, &post.id, false).unwrap().len(),
        0
    );
}

#[test]
fn create_comment_from_unexisting_post() {
    let client = init::clean_client();
    init::seed();

    let conn = init::database_connection();
    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    let comment_content = "I should not be sucessfully submitted!.";
    let response_status = send_comment_from_post_ko(
        &client,
        init::login_admin(),
        &unexisting_id,
        comment_content,
    );
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn create_comment_from_locked_post_normal_user() {
    let client = init::clean_client();
    init::seed();

    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(true, false, false);
    let response_status = send_comment_from_post_ko(
        &client, 
        auth_token_header, 
        &post.id, 
        "I cannot be submitted to a locked post by a normal user."
    );
    assert_eq!(response_status, Status::Forbidden);
}

#[test]
fn create_comment_from_locked_post_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(true, false, false);
    let comment_content = "Ahihi, you won!!!";
    let comment = send_comment_from_post(
        &client,
        login_admin(),
        &post.id,
        comment_content,
    );

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, comment_content);
}

#[test]
fn create_comment_from_hidden_post_normal_user() {
    let client = init::clean_client();
    init::seed();

    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, true, false);
    let response_status = send_comment_from_post_ko(
        &client, 
        auth_token_header, 
        &post.id, 
        "I cannot be submitted to a hidden post by a normal user."
    );
    assert_eq!(response_status, Status::Forbidden);
}

#[test]
fn create_comment_from_hidden_post_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, true, false);
    let comment_content = "An admin can still add a comment to a hidden post!";
    let returned_comment = send_comment_from_post(
        &client, 
        init::login_admin(), 
        &post.id, 
        comment_content
    );

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &returned_comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, comment_content);
}

#[test]
fn create_comment_from_soft_deleted_post() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, true);
    let comment_content = "No one can comment to a deleted post.";

    let response_status = send_comment_from_post_ko(
        &client, 
        init::login_admin(), 
        &post.id, 
        comment_content);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn create_comment_from_comment_admin() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Test <<positive>>";
    let reply = send_comment_from_comment(
        &client, 
        login_admin(), 
        &comment.id, 
        reply_content
    );

    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.post_id, post.id);
    assert_eq!(comment_entity.parent_id, Some(comment.id));
    assert_eq!(comment_entity.content, reply_content);
    assert_eq!(comment_entity.author_id, init::get_admin().id);
}

#[test]
fn create_comment_from_comment_normal_user() {
    let client = init::clean_client();
    init::seed();

    // init simple user
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Test <<positive>> too :D";
    let reply = send_comment_from_comment(
        &client,
        auth_token_header,
        &comment.id,
        reply_content
    );

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.post_id, post.id);
    assert_eq!(comment_entity.parent_id, Some(comment.id));
    assert_eq!(comment_entity.content, reply_content);
    assert_eq!(comment_entity.author_id, reply.author.id);
}

#[test]
fn create_duplicate_comments_from_comment() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let token = login_admin();
    let reply_content = "There are 2 comments like this!!";

    let reply1 = send_comment_from_comment(&client, token.clone(), &comment.id, reply_content);
    assert!(CommentEntity::by_id(&conn, &reply1.id).unwrap().is_some());

    thread::sleep(time::Duration::from_millis(1000));

    let reply2 = send_comment_from_comment(&client, token.clone(), &comment.id, reply_content);
    assert!(CommentEntity::by_id(&conn, &reply2.id).unwrap().is_some());

    assert_ne!(reply1.id, reply2.id);
    assert!(reply1.created_at < reply2.created_at);
}

// create a comment to a comment from a post (unauthenticated)
#[test]
fn create_comment_from_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let route = format!("/api/v1/comment/{}", comment.id);

    let reply_content = "Don't panic! Try your best!";
    let json_data = format!("{{ \"content\": \"{}\" }}", reply_content);

    let req = client.post(route).header(ContentType::JSON).body(json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        CommentEntity::by_comment_id(&conn, &comment.id, false)
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn create_comment_from_comment_bad_json() {
    let client = init::clean_client();
    init::seed();

    let conn = init::database_connection();
    let auth_header = init::login_admin();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let route = format!("/api/v1/comment/{}", comment.id);

    let reply_content = "Don't panic! Try your best!";
    let json_data = format!("{{ \"content\": \"{} }}", reply_content);

    let req = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_header)
        .body(json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
        CommentEntity::by_comment_id(&conn, &comment.id, false)
            .unwrap()
            .len(),
        0
    );
}

// create a comment to an unexisting comment from a post
#[test]
fn create_comment_from_unexisting_comment() {
    let client = init::clean_client();
    init::seed();

    let unexisting_id = init::get_unexisting_comment_id();
    let reply_content = "I should not be sucessfully submitted!.";
    let response =
        send_comment_from_comment_ko(&client, init::login_admin(), &unexisting_id, reply_content);
    assert_eq!(response.status(), Status::BadRequest);
}

// create a comment to a locked comment
#[test]
fn create_comment_from_locked_comment_normal_user() {
    let client = init::clean_client();
    init::seed();

    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, true, false, false);
    let reply_content = "Don't panic! Try your best!";

    let response =
        send_comment_from_comment_ko(&client, auth_token_header, &comment.id, reply_content);
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn create_comment_from_locked_comment_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, true, false, false);
    let reply_content = "Admin can still reply to a locked comment!";
    let reply = send_comment_from_comment(
        &client, 
        init::login_admin(), 
        &comment.id, 
        reply_content);
        
    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, reply_content);
}

// create a comment to a hidden comment from a post
#[test]
fn create_comment_from_hidden_comment_normal_user() {
    let client = init::clean_client();
    init::seed();

    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, true, false);
    let reply_content = "Normal user cannot reply to a hidden comment!";

    let response =
        send_comment_from_comment_ko(&client, auth_token_header, &comment.id, reply_content);
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn create_comment_from_hidden_comment_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, true, false);
    let reply_content = "Admin can still reply to a hidden comment!";

    let reply = send_comment_from_comment(
        &client, 
        init::login_admin(), 
        &comment.id, 
        reply_content
    );
    
    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, reply_content);
}

// create a comment to a soft-deleted comment from a post
#[test]
fn create_comment_from_deleted_comment() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, true);
    let reply_content = "No one can reply to a (soft) deleted comment!";

    let response =
        send_comment_from_comment_ko(&client, init::login_admin(), &comment.id, reply_content);
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn create_comment_from_comment_in_locked_post() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(true, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Admin can still reply to a comment on a locked post!";
    let reply = send_comment_from_comment(
        &client, 
        login_admin(),
        &comment.id,
        reply_content
    );

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, reply_content);
}

#[test]
fn create_comment_from_comment_in_locked_post_missing_capability() {
    let client = init::clean_client();
    init::seed();

    let (user, password) = init::get_user(true);
    let post = init::get_post_entity(true, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Don't panic! Try your best!";

    let response = send_comment_from_comment_ko(
        &client,
        init::login(&user.email, &password),
        &comment.id,
        reply_content,
    );
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn create_comment_from_comment_in_hidden_post_normal_user() {
    let client = init::clean_client();
    init::seed();

    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, true, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Don't panic! Try your best!";
    let response = send_comment_from_comment_ko(
        &client, 
        auth_token_header, 
        &comment.id, 
        reply_content
    );
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn create_comment_from_comment_in_hidden_post_admin() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, true, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Admin can still reply to a comment on a locked post!";
    let reply = send_comment_from_comment(
        &client, 
        login_admin(),
        &comment.id,
        reply_content
    );

    let conn = init::database_connection();
    let comment_entity = CommentEntity::by_id(&conn, &reply.id).unwrap().unwrap();
    assert_eq!(comment_entity.content, reply_content);
}

#[test]
fn create_comment_from_comment_in_deleted_post() {
    let client = init::clean_client();
    init::seed();

    let post = init::get_post_entity(false, false, true);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let reply_content = "Don't panic! Try your best!";

    let response =
        send_comment_from_comment_ko(&client, init::login_admin(), &comment.id, reply_content);
    assert_eq!(response.status(), Status::BadRequest);
}

// get all comments from a post
// get all comments from an unexisting post
// get all comments from a soft-deleted post
// get all comments from a hidden post (admin)
// get all comments from a hidden post (unauthenticated)
// get all comments from a locked post -> ok
// get all comments ordered by time asc
// get all comments ordered by time desc
// get all comments ordered by score asc
// get all comments ordered by score desc
// get all comments with limit & offset

#[test]
fn get_normal_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_unauth_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_normal_comment_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_normal_user_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_normal_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_admin_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

// get a specific comment (unexisting id)
#[test]
fn get_unexisting_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let unexisting_id = init::get_unexisting_comment_id();

    let response_status = get_comment_unauth_ko(&client, &unexisting_id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_unexisting_comment_authed() {
    let client = init::clean_client();
    init::seed();
    let unexisting_id = init::get_unexisting_comment_id();

    let response_status = get_comment_normal_user_ko(&client, &unexisting_id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_comment_from_locked_post_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(true, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_unauth_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_comment_from_locked_post_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(true, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_normal_user_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_comment_from_locked_post_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(true, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_admin_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

// get a specific comment from a hidden post (unauthenticated)
#[test]
fn get_comment_from_hidden_post_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, true, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let response_status = get_comment_unauth_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_comment_from_hidden_post_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, true, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let response_status = get_comment_normal_user_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

// get a specific comment from a hidden post (admin)
#[test]
fn get_comment_from_hidden_post_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, true, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let returned_comment = get_comment_admin_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

// get a specific comment from a soft-deleted post
#[test]
fn get_comment_from_deleted_post_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, true);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let response_status = get_comment_unauth_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_comment_from_deleted_post_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, true);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let response_status = get_comment_normal_user_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_comment_from_deleted_post_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, true);
    let a_comment = init::get_comment_entity(a_post.id, false, false, false);

    let response_status = get_comment_admin_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

// get a specific locked comment -> ok
#[test]
fn get_locked_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, true, false, false);

    let returned_comment = get_comment_unauth_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_locked_comment_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, true, false, false);

    let returned_comment = get_comment_normal_user_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

#[test]
fn get_locked_comment_post_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, true, false, false);

    let returned_comment = get_comment_admin_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

// get a specific hidden comment
#[test]
fn get_hidden_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, true, false);

    let response_status = get_comment_unauth_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_hidden_comment_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, true, false);

    let response_status = get_comment_normal_user_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_hidden_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, true, false);

    let returned_comment = get_comment_admin_ok(&client, &a_comment.id);
    assert_eq!(a_comment.id, returned_comment.id);
}

// get a specific soft-deleted comment
#[test]
fn get_deleted_comment_unauthenticated() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, true);

    let response_status = get_comment_unauth_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_deleted_comment_normal_user() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, true);

    let response_status = get_comment_normal_user_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn get_deleted_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let a_post = init::get_post_entity(false, false, false);
    let a_comment = init::get_comment_entity(a_post.id, false, false, true);

    let response_status = get_comment_admin_ko(&client, &a_comment.id);
    assert_eq!(response_status, Status::BadRequest);
}

#[test]
fn update_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment_entity = init::get_comment_entity(post.id, false, false, false);

    let content = "I am so unique !";
    assert_ne!(&comment_entity.content, content);

    let comment = update_comment(&client, init::login_admin(), &comment_entity.id, content);

    assert_eq!(&comment.content, content);
}

#[test]
fn update_comment_author() {
    let client = init::clean_client();
    init::seed();
    // init simple user
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post = init::get_post_entity(false, false, false);
    let comment = send_comment_from_post(
        &client,
        auth_token_header.clone(),
        &post.id,
        "Normal user should be able to post a comment like this!!!",
    );

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);

    let new_comment = update_comment(&client, auth_token_header.clone(), &comment.id, content);

    assert_eq!(&new_comment.content, content);
}

#[test]
fn update_comment_non_author() {
    let client = init::clean_client();
    init::seed();
    // init simple user
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let (other_user, other_passord) = init::get_user(true);
    let other_auth_token = init::login(&other_user.email, &other_passord);

    let post = init::get_post_entity(false, false, false);
    let comment = send_comment_from_post(
        &client,
        auth_token_header.clone(),
        &post.id,
        "Normal user should be able to post a comment like this!!!",
    );

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);

    let route = format!("{}/{}", COMMENT_ROUTE, &comment.id);
    let json_data = format!("{{ \"content\": \"{}\" }}", content);
    let response = client
        .put(route)
        .header(other_auth_token)
        .header(ContentType::JSON)
        .body(json_data)
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);

    assert_ne!(&comment.content, content);
}

#[test]
fn update_comment_non_authenticated() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);

    let route = format!("{}/{}", COMMENT_ROUTE, &comment.id);
    let json_data = format!("{{ \"content\": \"{}\" }}", content);
    let response = client
        .put(route)
        .header(ContentType::JSON)
        .body(json_data)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);

    assert_ne!(&comment.content, content);
}

#[test]
fn update_comment_unexisting_id() {
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

    let content = "I am so unique !";
    let route = format!("{}/{}", COMMENT_ROUTE, &unexisting_id);
    let json_data = format!("{{ \"content\": \"{}\" }}", content);
    let response = client
        .put(route)
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(json_data)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn update_comment_deleted() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, true);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);

    let route = format!("{}/{}", COMMENT_ROUTE, &comment.id);
    let json_data = format!("{{ \"content\": \"{}\" }}", content);
    let response = client
        .put(route)
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(json_data)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    assert_ne!(&comment.content, content);
}

#[test]
fn update_comment_hidden_post_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, true, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);
    let new_comment = update_comment(&client, init::login_admin(), &comment.id, content);
    assert_eq!(&new_comment.content, content);
}

#[test]
fn update_comment_locked_post_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(true, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);
    let new_comment = update_comment(&client, init::login_admin(), &comment.id, content);
    assert_eq!(&new_comment.content, content);
}

#[test]
fn update_comment_hidden_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, true, false);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);
    let new_comment = update_comment(&client, init::login_admin(), &comment.id, content);
    assert_eq!(&new_comment.content, content);
}

#[test]
fn update_comment_locked_comment_admin() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, true, false, false);

    let content = "I am so unique !";
    assert_ne!(&comment.content, content);
    let new_comment = update_comment(&client, init::login_admin(), &comment.id, content);
    assert_eq!(&new_comment.content, content);
}

// update a comment from a hidden post (author) -> nok
// update a hidden comment (author) -> nok
// update a comment with malformed json

// delete a comment (admin)
// delete a comment (author)
// delete a comment (non-author)
// delete a comment (unauthenticated)
// delete a comment unexisting id
// delete a comment from a soft-deleted post
// delete a comment from a hidden post (admin) -> ok
// delete a comment from a hidden post (author) -> nok
// delete a comment from a locked post (admin) -> nok
// delete a soft-deleted comment
// delete an hidden comment (admin) -> ok
// delete an hidden comment (author) -> nok
// delete a locked comment (admin) -> nok
