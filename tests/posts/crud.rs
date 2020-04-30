use rocket::http::Status;

use super::super::init;
use super::utils;

use rocket::http::ContentType;
use unanimitylibrary::database::models::prelude::*;

const POSTS_ROUTE: &'static str = "/api/v1/posts";
const POST_ROUTE: &'static str = "/api/v1/post";

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

#[test]
pub fn create_post() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
}

#[test]
pub fn create_post_simple_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
}

#[test]
pub fn create_post_unauthenticated_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Forbidden);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
pub fn create_post_missing_attribute() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!("{{\"title\": \"{}\"}}", new_post_title);

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
pub fn create_post_bad_json() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": {}}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
pub fn create_duplicate_post() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );

    let req1 = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data.clone());
    let mut response1 = req1.dispatch();

    assert_eq!(response1.status(), Status::Ok);
    let data1 = response1.body_string().unwrap();
    let p1: Post = serde_json::from_str(&data1).unwrap();

    assert_eq!(p1.title, new_post_title);
    assert_eq!(p1.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p1.id).unwrap().is_some());
    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        1
    );

    let req2 = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data.clone());
    let mut response2 = req2.dispatch();

    assert_eq!(response2.status(), Status::Ok);
    let data2 = response2.body_string().unwrap();
    let p2: Post = serde_json::from_str(&data2).unwrap();

    assert_eq!(p2.title, new_post_title);
    assert_eq!(p2.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p2.id).unwrap().is_some());
    assert_ne!(p1.id, p2.id);
    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        2
    );
}

#[test]
pub fn create_post_simple_user_with_tags() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_tags = vec!["even".to_string(), "odd".to_string()];

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\", \"tags\": [{}]}}",
        new_post_title,
        new_post_content,
        new_post_tags
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
    let tags = RelPostTagEntity::tags_by_post_id(&conn, &p.id)
        .unwrap()
        .iter()
        .map(|t| t.label.to_string())
        .collect::<Vec<String>>();

    for t in new_post_tags {
        assert!(tags.contains(&t));
    }
}

#[test]
pub fn create_post_simple_user_with_new_tags() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_tags = vec!["frankyvincent".to_string(), "lerestaurant".to_string()];

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\", \"tags\": [{}]}}",
        new_post_title,
        new_post_content,
        new_post_tags
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
    let tags = RelPostTagEntity::tags_by_post_id(&conn, &p.id)
        .unwrap()
        .iter()
        .map(|t| t.label.to_string())
        .collect::<Vec<String>>();

    for t in new_post_tags {
        assert!(tags.contains(&t));
    }
}

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
