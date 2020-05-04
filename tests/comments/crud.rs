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

    let comment = send_comment_from_post(&client, login_admin(), &post.id, "FIIIIIRST!!!!");

    let comment_entity = CommentEntity::by_id(&conn, &comment.id).unwrap().unwrap();
    assert_eq!(comment_entity.id, comment.id);
    assert_eq!(comment_entity.content, comment.content);
    assert_eq!(comment_entity.author_id, comment.author.id);
    assert_eq!(comment_entity.author_id, init::get_admin().id);
}

// create a comment to a comment from a post
// create a comment from a post (unauthenticated)
// create a comment to a comment from a post (unauthenticated)
// create a comment with a malformed json
// create a comment from an unexisting post
// create a comment from an hidden post
// create a comment from a soft-deleted post
// create a comment from a locked post
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
