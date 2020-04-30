use rocket::http::Status;

use super::super::init;
use super::utils;

use unanimitylibrary::database::models::prelude::Post;
use unanimitylibrary::lib::seeds::posts::seed_test_posts;

const POSTS_ROUTE: &'static str = "/api/v1/posts";

#[test]
fn read_all_posts_while_logged_in() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // perform request
    let req = client.get(POSTS_ROUTE).header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 6 post, the 5 normals & the locked one
    assert_eq!(posts.len(), 6);

    // assert we can see the locked one
    assert_eq!(posts.iter().filter(|p| p.locked).count(), 1);

    // assert we cant see the hidden one
    assert_eq!(posts.iter().filter(|p| p.hidden).count(), 0);

    // assert we cant see a deleted
    assert_eq!(posts.iter().filter(|p| p.deleted).count(), 0);

    // the others must be "normal"
    assert_eq!(
        posts
            .iter()
            .filter(|p| !p.hidden && !p.hidden && !p.locked)
            .count(),
        5
    );
}

#[test]
fn read_all_posts_while_logged_in_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login_admin();

    // perform request
    let req = client.get(POSTS_ROUTE).header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 7 post, the 5 normals, the locked one & the hidden one
    assert_eq!(posts.len(), 7);

    // assert we can see the locked one
    assert_eq!(posts.iter().filter(|p| p.locked).count(), 1);

    // assert we can see the hidden one
    assert_eq!(posts.iter().filter(|p| p.hidden).count(), 1);

    // assert we cant see a deleted
    assert_eq!(posts.iter().filter(|p| p.deleted).count(), 0);

    // the others must be "normal"
    assert_eq!(
        posts
            .iter()
            .filter(|p| !p.hidden && !p.hidden && !p.locked)
            .count(),
        5
    );
}

#[test]
fn read_all_posts_without_being_logged_in() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // perform request
    let req = client.get(POSTS_ROUTE);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 6 post, the 5 normals, the hidden & the locked one
    assert_eq!(posts.len(), 6);

    // assert we can see the locked one
    assert_eq!(posts.iter().filter(|p| p.locked).count(), 1);

    // assert we can see the hidden one
    assert_eq!(posts.iter().filter(|p| p.hidden).count(), 0);

    // assert we cant see a deleted
    assert_eq!(posts.iter().filter(|p| p.deleted).count(), 0);

    // the others must be "normal"
    assert_eq!(
        posts
            .iter()
            .filter(|p| !p.hidden && !p.hidden && !p.locked)
            .count(),
        5
    );
}

// read all posts with a search term
// read all posts related to a tag
// read all posts with a sorting criteria
// read all post of a certain type
// read all post with limit and offset

// create a post (admin)
// create a post (wrong permission)
// create a post (invalid json)
// create a post (existing title)
// create a post (multiple existing tags)
// create a post (with new tags)

// read a post
// read a post (invalid id)
// read a soft-deleted post (same response as invalid id)
// read an hidden post (???)

// update a post (admin)
// update a post with appropriate author
// update a post with a different author
// update a post (invalid post id)
// update a soft-deleted post (same response as invalid id)
// update a post (invalid json)
// update a locked post (admin)
// update a locked post (author) -> error
// update an hidden post (admin)
// update an hidden post (author) -> error

// delete a post (admin)
// delete a post with appropriate author
// delete a post with a different author
// delete a post (invalid post id)
// delete a soft-deleted post
// delete a locked post (admin)
// delete a locked post (author) -> error
// delete an hidden post (admin)
// delete an hidden post (author) -> error
