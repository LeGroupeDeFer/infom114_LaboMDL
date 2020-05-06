use std::{thread, time};

use super::super::init;
use rocket::http::{ContentType, Status};
use unanimitylibrary::database::models::prelude::*;

use super::helper::*;
use crate::init::login_admin;

#[test]
fn create_comment_from_post() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let post = init::get_post_entity(false, false, false);

    let comment = send_comment_from_post(&client, login_admin(), &post.id, "FIIIIIRST!!!");

    let comment_entity = CommentEntity::by_id(&conn, &comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.id, comment.id);
    assert_eq!(comment_entity.post_id, post.id);
    assert_eq!(comment_entity.parent_id, None);
    assert_eq!(comment_entity.content, comment.content);
    assert_eq!(comment_entity.author_id, comment.author.id);
    assert_eq!(comment_entity.author_id, init::get_admin().id);
}

#[test]
fn create_comment_from_post_duplicate_details() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let post = init::get_post_entity(false, false, false);
    let token = login_admin();
    let comment_content = "There are 2 comments like this!!";
    
    let comment1 = send_comment_from_post(&client, token.clone(), &post.id, comment_content);
    assert!(CommentEntity::by_id(&conn, &comment1.id).unwrap().is_some());
    
    thread::sleep(time::Duration::from_millis(10000));

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

    let req = client
        .post(route)
        .header(ContentType::JSON)
        .body(json_data);

    let response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        CommentEntity::by_post_id(&conn, &post.id, false).unwrap().len(), 
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

    let req = client
        .post(route)
        .header(ContentType::JSON)
        .header(auth_header)
        .body(bad_json);

    let response = req.dispatch();
    assert_eq!(response.status(), Status::BadRequest);
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
    let response_status = send_comment_from_unavailable_post(
        &client,
        init::login_admin(),
        &unexisting_id,
        comment_content
    );
    assert_eq!(response_status, Status::BadRequest); 
}

#[test]
fn create_comment_from_locked_post() {
    let client = init::clean_client();
    init::seed();
    
    let post = init::get_post_entity(true, false, false);
    let route = format!("/api/v1/post/{}/comment", post.id);
    let comment_content = "I am a comment, I cannot be submitted to a hidden post.";

    let response_status = send_comment_from_unavailable_post(
        &client,
        init::login_admin(),
        &post.id,
        comment_content
    );
    assert_eq!(response_status, Status::BadRequest); 
}

// create a comment from an hidden post
#[test]
fn create_comment_from_hidden_post() {
    let client = init::clean_client();
    init::seed();
    

    let post = init::get_post_entity(false, true, false);
    let route = format!("/api/v1/post/{}/comment", post.id);
    let comment_content = "I am a comment, I cannot be submitted to a hidden post.";

    let response_status = send_comment_from_unavailable_post(
        &client,
        init::login_admin(),
        &post.id,
        comment_content
    );
    assert_eq!(response_status, Status::BadRequest); 
}

// create a comment from a soft-deleted post
#[test]
fn create_comment_from_soft_deleted_post() {
    let client = init::clean_client();
    init::seed();
    
    let post = init::get_post_entity(false, false, true);
    let route = format!("/api/v1/post/{}/comment", post.id);
    let comment_content = "I am a comment, I cannot be submitted to a soft-deleted post.";

    let response_status = send_comment_from_unavailable_post(
        &client,
        init::login_admin(),
        &post.id,
        comment_content
    );
    assert_eq!(response_status, Status::BadRequest); 
}

// create a comment to a comment from a post
// create a comment to a comment from a post (unauthenticated)
// create a comment to an unexisting comment from a post

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

// get a specific comment
// get a specific comment (unexisting id)
// get a specific comment from a soft-deleted post
// get a specific comment from a hidden post (admin)
// get a specific comment from a hidden post (unauthenticated)
// get a specific soft-deleted comment
// get a specific locked comment -> ok
// get a specific hidden comment

// update a comment (admin)
// update a comment (author)
// update a comment (non-author)
// update a comment (unauthenticated)
// update a comment unexisting id
// update a comment from a soft-deleted post
// update a comment from a hidden post (admin) -> ok
// update a comment from a hidden post (author) -> nok
// update a comment from a locked post (admin) -> nok
// update a hidden comment (admin) -> ok
// update a hidden comment (author) -> nok
// update a locked comment (admin) -> nok
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
