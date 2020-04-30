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

#[test]
fn read_a_post() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let existing_post_entity = init::get_post_entity(false, false, false);

    // perform request
    let req = client.get(format!("{}/{}", POST_ROUTE, existing_post_entity.id));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let post: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(post.id, existing_post_entity.id);
}

#[test]
fn read_a_post_as_authenticated() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let post_entity = init::get_post_entity(false, false, false);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, post_entity.id))
        .header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let post: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(post.id, post_entity.id);
}

#[test]
fn read_a_post_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let auth_token_header = init::login_admin();
    let existing_post_entity = init::get_post_entity(false, false, false);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let post: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(post.id, existing_post_entity.id);
}

#[test]
fn read_a_post_invalid_id() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    // perform request
    let req = client.get(format!("{}/{}", POST_ROUTE, unexisting_id));
    let mut response = req.dispatch();

    //check the answer is a Bad request
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn read_a_post_soft_deleted() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let auth_token_header = init::login_admin();
    let existing_post_entity = init::get_post_entity(false, false, true);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn read_a_post_hidden_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let auth_token_header = init::login_admin();
    let existing_post_entity = init::get_post_entity(false, true, false);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let post: Post = serde_json::from_str(&data).unwrap();

    assert_eq!(post.id, existing_post_entity.id);
}

#[test]
fn read_a_post_hidden_as_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);
    let existing_post_entity = init::get_post_entity(false, true, false);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn update_post_as_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );
    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data);
    let mut response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);
    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_eq!(updated_post.title, updated_title);
    assert_eq!(updated_post.content, updated_content);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn update_post_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );
    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data);
    let mut response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // update
    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(init::login_admin())
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);
    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_eq!(updated_post.title, updated_title);
    assert_eq!(updated_post.content, updated_content);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn update_post_as_stun_fest_random() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, false, false);

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated_post.title, updated_title);
    assert_ne!(not_updated_post.content, updated_content);
    assert_eq!(not_updated_post.id, p.id);
}

#[test]
fn update_post_invalid_id() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &unexisting_id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::BadRequest);
}

#[test]
fn update_post_deleted() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, false, true);

    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::BadRequest);

    let not_updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_eq!(not_updated_post.title, p.title);
    assert_eq!(not_updated_post.content, p.content);
    assert_eq!(not_updated_post.id, p.id);
}

#[test]
fn update_post_invalid_json() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, false, false);
    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":{}}}",
        updated_title, updated_content
    );

    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::BadRequest);

    let not_updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated_post.title, updated_title);
    assert_ne!(not_updated_post.content, updated_content);
    assert_eq!(not_updated_post.id, p.id);
}

#[test]
fn update_post_missing_attribute() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, false, false);
    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!("{{\"title\": \"{}\"}}", updated_title);

    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::UnprocessableEntity);

    let not_updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated_post.title, updated_title);
    assert_ne!(not_updated_post.content, updated_content);
    assert_eq!(not_updated_post.id, p.id);
}

#[test]
fn update_post_locked_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(true, false, false);

    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);

    let updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_eq!(updated.title, updated_title);
    assert_eq!(updated.content, updated_content);
    assert_ne!(updated.title, p.title);
    assert_ne!(updated.content, p.content);
    assert_eq!(updated.id, p.id);
}

#[test]
fn update_post_locked_as_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(true, false, false);

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated.title, updated_title);
    assert_ne!(not_updated.content, updated_content);
    assert_eq!(not_updated.title, p.title);
    assert_eq!(not_updated.content, p.content);
    assert_eq!(not_updated.id, p.id);
}

#[test]
fn update_post_locked_as_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );
    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data);
    let mut response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // lock the post
    let mut post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_lock(&conn).unwrap();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated.title, updated_title);
    assert_ne!(not_updated.content, updated_content);
    assert_eq!(not_updated.title, p.title);
    assert_eq!(not_updated.content, p.content);
    assert_eq!(not_updated.id, p.id);
}

#[test]
fn update_post_hidden_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, true, false);

    let auth_token_header = init::login_admin();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);

    let updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_eq!(updated.title, updated_title);
    assert_eq!(updated.content, updated_content);
    assert_ne!(updated.title, p.title);
    assert_ne!(updated.content, p.content);
    assert_eq!(updated.id, p.id);
}

#[test]
fn update_post_hidden_as_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, true, false);

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated.title, updated_title);
    assert_ne!(not_updated.content, updated_content);
    assert_eq!(not_updated.title, p.title);
    assert_eq!(not_updated.content, p.content);
    assert_eq!(not_updated.id, p.id);
}

#[test]
fn update_post_hidden_as_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\"}}",
        new_post_title, new_post_content
    );
    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header.clone())
        .body(post_json_data);
    let mut response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    let data = response.body_string().unwrap();
    let p: Post = serde_json::from_str(&data).unwrap();
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // hide the post
    let mut post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_visibility(&conn).unwrap();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\"}}",
        updated_title, updated_content
    );
    let update_req = client
        .put(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(update_json_data);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert_ne!(not_updated.title, updated_title);
    assert_ne!(not_updated.content, updated_content);
    assert_eq!(not_updated.title, p.title);
    assert_eq!(not_updated.content, p.content);
    assert_eq!(not_updated.id, p.id);
}

// delete a post (admin)
// delete a post with appropriate author
// delete a post with a different author
// delete a post (invalid post id)
// delete a soft-deleted post
// delete a locked post (admin)
// delete a locked post (author) -> error
// delete an hidden post (admin)
// delete an hidden post (author) -> error
