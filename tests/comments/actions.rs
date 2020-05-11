use super::super::init;
use super::helper::*;
use rocket::http::Status;

// use rocket::http::ContentType;
// use unanimitylibrary::database::models::prelude::*;
// upvote a comment (+1)
#[test]
fn upvote_comment() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    let resp = send_vote(&client, auth_token_header.clone(), &comment.id, 1);
    assert_eq!(resp.status(), Status::Ok);
    let tmp_comment = get_comment(&client, auth_token_header.clone(), &comment.id);
    assert_eq!(tmp_comment.score, 1);
    assert_eq!(tmp_comment.user_vote.unwrap(), 1);
    assert_eq!(tmp_comment.votes, 1);
}

// upvote a comment (0)
// upvote a comment (-1)
// upvote a comment (2) (error)
// upvote a comment without being logged in
// upvote a comment with malformed json
// upvote a soft-deleted comment
// upvote a comment (wrong id)
// upvote a locked comment
// upvote an hidden comment

// hide and unhide a comment
// hide a comment (invalid id)
// hide a comment (invalid capability)

// lock and unlock a comment
// lock a comment (invalid id)
// lock a comment (invalid capability)

// report a comment
// report a comment (invalid id)
// report a comment (invalid capability)
// report same comment twice
