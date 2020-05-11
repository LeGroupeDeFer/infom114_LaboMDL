use super::super::init;
use super::helper::*;

// create a poll with correct capabilities
#[test]
fn create_a_post_poll() {
    let client = init::clean_client();
    init::seed();
    let auth_token = init::login_admin();

    let post = create_a_poll_post(
        &client,
        auth_token.clone(),
        "test poll",
        &["response 1", "response 2", "response 3"],
    );

    let _poll_info = get_poll_info(&client, auth_token.clone(), &post.id);
}

#[test]
fn vote_for_a_poll_post() {
    let client = init::clean_client();
    init::seed();
    let auth_token = init::login_admin();

    let post = create_a_poll_post(
        &client,
        auth_token.clone(),
        "test poll",
        &["response 1", "response 2", "response 3"],
    );

    let poll_info = get_poll_info(&client, auth_token.clone(), &post.id);
    assert!(poll_info.user_answer.is_none());
    let answer = poll_info.answers.get(0).unwrap();
    assert_eq!(answer.count, 0);

    let voted_post = send_poll_vote(&client, auth_token.clone(), &post.id, &answer.id);
    assert!(voted_post.user_answer.is_some());
    assert_eq!(voted_post.user_answer.unwrap().id, answer.id);
    assert_eq!(voted_post.answers.get(0).unwrap().count, 1);
}

// create a poll unauthenticated
// create a poll without capabilities
// create a poll invalid form
// create a poll non json

// update a info post into a poll post
// update an info post into a poll post without capabilities

// answer to a poll post
// answer to a poll post without being authenticated

// get a poll post
// get a poll post unauthenticated

// remove a poll post
