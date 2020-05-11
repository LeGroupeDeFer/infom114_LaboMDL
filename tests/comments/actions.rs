use super::super::init;
use super::helper::*;
use rocket::http::Status;

// use rocket::http::ContentType;
use unanimitylibrary::database::models::prelude::*;

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

#[test]
fn report_a_comment() {
    let client = init::clean_client();
    init::seed();
    let post = init::get_post_entity(false, false, false);
    let comment = init::get_comment_entity(post.id, false, false, false);
    let conn = init::database_connection();
    let mut tmp_comment: Comment;
    let auth_token = init::login_admin();

    let (user1, password) = init::get_user(true);
    let auth_token_user1 = init::login(&user1.email, &password);
    let (user2, password) = init::get_user(true);
    let auth_token_user2 = init::login(&user2.email, &password);
    let (user3, password) = init::get_user(true);
    let auth_token_user3 = init::login(&user3.email, &password);

    // assert comment is not reported
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 0);

    // report comment by admin
    let r0 = toggle_report(&client, auth_token.clone(), &comment.id, Some("random reason"));
    assert_eq!(r0.status(), Status::Ok);

    // assert comment is reported by admin
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 1);
    // assert the post is not reported by user1
    tmp_comment = get_comment(&client, auth_token_user1.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user1.id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 1);

    // report comment with user 1
    let r1 = toggle_report(
        &client,
        auth_token_user1.clone(),
        &comment.id,
        Some("random reason"),
    );
    assert_eq!(r1.status(), Status::Ok);

    // assert comment is reported by admin & user 1
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
    tmp_comment = get_comment(&client, auth_token_user1.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user1.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
    // assert post is not reported by user 3
    tmp_comment = get_comment(&client, auth_token_user3.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user3.id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);

    // report post with user 2
    let r2 = toggle_report(
        &client,
        auth_token_user2.clone(),
        &comment.id,
        Some("random reason"),
    );
    assert_eq!(r2.status(), Status::Ok);

    // assert admin, user 1 and user 2 do have a report
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    tmp_comment = get_comment(&client, auth_token_user1.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user1.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    tmp_comment = get_comment(&client, auth_token_user2.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user2.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    // assert comment is not reported by user 3
    tmp_comment = get_comment(&client, auth_token_user3.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user3.id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);

    // user 3 try to remove his (unexisting) report
    let r3 = toggle_report(&client, auth_token_user3.clone(), &comment.id, None);
    assert_eq!(r3.status(), Status::BadRequest);

    // assert admin, user 1 and user 2 do have a report
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    tmp_comment = get_comment(&client, auth_token_user1.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user1.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    tmp_comment = get_comment(&client, auth_token_user2.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user2.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);
    // assert comment is not reported by user 3
    tmp_comment = get_comment(&client, auth_token_user3.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user3.id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 3);

    // remove a report comment with admin
    let r4 = toggle_report(&client, auth_token.clone(), &comment.id, None);
    assert_eq!(r4.status(), Status::Ok);

    // assert user 1 and user 2 do have a report
    tmp_comment = get_comment(&client, auth_token_user1.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user1.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
    tmp_comment = get_comment(&client, auth_token_user2.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user2.id);
    assert!(tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
    // assert comment is not reported by admin neither user 3
    tmp_comment = get_comment(&client, auth_token.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &init::get_admin().id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
    tmp_comment = get_comment(&client, auth_token_user3.clone(), &comment.id);
    tmp_comment.set_user_info(&conn, &user3.id);
    assert!(!tmp_comment.user_flag.unwrap());
    assert_eq!(tmp_comment.flags, 2);
}

// report a comment (invalid id)
// report a comment (invalid capability)
// report same comment twice
