use super::super::init;
use chrono::NaiveDateTime;
use rocket::http::ContentType;
use rocket::http::Status;
use unanimitylibrary::database::models::prelude::*;
use unanimitylibrary::lib::seeds;

const POSTS_ROUTE: &'static str = "/api/v1/posts";
const POST_ROUTE: &'static str = "/api/v1/post";

fn get_posts_limit_and_offset(
    client: &rocket::local::Client,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Vec<Post> {
    let route: String = if limit.is_some() && offset.is_some() {
        format!(
            "{}?limit={}&offset={}",
            POSTS_ROUTE,
            limit.unwrap(),
            offset.unwrap()
        )
    } else if let Some(l) = limit {
        format!("{}?limit={}", POSTS_ROUTE, l)
    } else if let Some(o) = offset {
        format!("{}?offset={}", POSTS_ROUTE, o)
    } else {
        POSTS_ROUTE.to_string()
    };

    let req = client.get(route);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

fn send_create_post(
    client: &rocket::local::Client,
    auth_token: rocket::http::Header<'static>,
    post_title: &str,
    post_content: &str,
    post_kind: &str,
    tags: &[&str],
) -> Post {
    let json_post = format!(
        "{{ \
    \"title\": \"{}\",\
    \"content\": \"{}\",\
    \"kind\" : \"{}\",\
    \"tags\" : [{}]
    }}",
        post_title,
        post_content,
        post_kind,
        tags.iter()
            .map(|&t| format!("\"{}\"", t))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let mut response = client
        .post(POST_ROUTE)
        .header(auth_token)
        .header(ContentType::JSON)
        .body(json_post)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    serde_json::from_str(&data).unwrap()
}

// todo : read all post of a certain type

#[test]
fn read_all_posts_while_logged_in() {
    // clean database
    let client = init::clean_client();
    init::seed();

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
fn read_all_no_posts() {
    // clean database
    let client = init::clean_client();
    let conn = init::database_connection();
    seeds::roles::seed_roles_and_capabilities(&conn);
    seeds::tags::seed_tags(&conn);

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
    // we want a total of 0 post
    assert_eq!(posts.len(), 0);
}

#[test]
fn read_all_no_posts_unauthenticated() {
    // clean database
    let client = init::clean_client();

    // perform request
    let req = client.get(POSTS_ROUTE);
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 0 post
    assert_eq!(posts.len(), 0);
}

#[test]
fn read_all_post_query_tags_even() {
    // clean database
    let client = init::clean_client();
    init::seed();

    // perform request
    let req = client.get(format!("{}?tag=even", POSTS_ROUTE));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 0 post
    assert_eq!(posts.len(), 2);

    assert_eq!(
        posts
            .iter()
            .filter(|&p| p.tags.contains(&"even".to_string()))
            .count(),
        posts.len()
    )
}

#[test]
fn read_all_post_query_search_lock_in_title() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let search_term = "lock";

    // perform request
    let req = client.get(format!("{}?search={}", POSTS_ROUTE, &search_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 0 post
    assert_eq!(posts.len(), 1);

    assert!(posts[0]
        .title
        .to_lowercase()
        .contains(&search_term.to_string()));
}

#[test]
fn read_all_post_query_search_valid_in_title() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let search_term = "valid";
    // perform request
    let req = client.get(format!("{}?search={}", POSTS_ROUTE, &search_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 0 post
    assert_eq!(posts.len(), 5);

    assert_eq!(
        posts
            .iter()
            .filter(|&p| p.title.to_lowercase().contains(&search_term.to_string()))
            .count(),
        posts.len()
    );
}

#[test]
fn read_all_post_query_tags_odd() {
    // clean database
    let client = init::clean_client();
    init::seed();

    // perform request
    let req = client.get(format!("{}?tag=odd", POSTS_ROUTE));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the answer data is what we wanted
    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    // we want a total of 0 post
    assert_eq!(posts.len(), 3);

    assert_eq!(
        posts
            .iter()
            .filter(|&p| p.tags.contains(&"odd".to_string()))
            .count(),
        posts.len()
    )
}

#[test]
fn read_all_post_query_sort_by_invalid() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let sorting_term = "invalid";

    // perform request
    let req = client.get(format!("{}?sort={}", POSTS_ROUTE, sorting_term));
    let response = req.dispatch();

    //check the answer is a bad request
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn read_all_post_query_sort_by_new() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let sorting_term = "new";

    // perform request
    let req = client.get(format!("{}?sort={}", POSTS_ROUTE, sorting_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    let mut desc_time = NaiveDateTime::from_timestamp(i32::MAX as i64, 0);

    for post in posts {
        let comparable_time =
            NaiveDateTime::parse_from_str(post.created_at.as_ref(), "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(desc_time >= comparable_time);
        if comparable_time < desc_time {
            desc_time = comparable_time;
        }
    }
}

#[test]
fn read_all_post_query_sort_by_old() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let sorting_term = "old";

    // perform request
    let req = client.get(format!("{}?sort={}", POSTS_ROUTE, sorting_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    let mut asc_time = NaiveDateTime::from_timestamp(i32::MIN as i64, 0);

    for post in posts {
        let comparable_time =
            NaiveDateTime::parse_from_str(post.created_at.as_ref(), "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(asc_time <= comparable_time);
        if comparable_time > asc_time {
            asc_time = comparable_time;
        }
    }
}

#[test]
fn read_all_post_query_sort_by_score_desc() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let p = init::get_post_entity(false, false, false);
    super::actions::send_vote(&client, init::login_admin(), &p.id, 1);

    let sorting_term = "top";

    // perform request
    let req = client.get(format!("{}?sort={}", POSTS_ROUTE, sorting_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    let mut asc_score = i64::MAX;

    for post in posts {
        assert!(asc_score >= post.score);
        if post.score < asc_score {
            asc_score = post.score;
        }
    }
}

#[test]
fn read_all_post_query_sort_by_score_asc() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let p = init::get_post_entity(false, false, false);
    super::actions::send_vote(&client, init::login_admin(), &p.id, 1);

    let sorting_term = "low";

    // perform request
    let req = client.get(format!("{}?sort={}", POSTS_ROUTE, sorting_term));
    let mut response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    let data = response.body_string().unwrap();
    let posts: Vec<Post> = serde_json::from_str(&data).unwrap();
    let mut asc_score = i64::MIN;

    for post in posts {
        assert!(asc_score <= post.score);
        if post.score > asc_score {
            asc_score = post.score;
        }
    }
}

#[test]
fn read_all_post_query_limit_and_offset() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let mut tmp_posts: Vec<Post>;

    tmp_posts = get_posts_limit_and_offset(&client, Some(2), None);
    assert_eq!(tmp_posts.len(), 2);

    let mut posts_iter = tmp_posts.iter();
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Valid post #1");
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Valid post #2");
    assert!(posts_iter.next().is_none());

    tmp_posts = get_posts_limit_and_offset(&client, Some(1), Some(2));
    assert_eq!(tmp_posts.len(), 1);

    posts_iter = tmp_posts.iter();
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Valid post #3");
    assert!(posts_iter.next().is_none());

    tmp_posts = get_posts_limit_and_offset(&client, None, Some(3));
    assert_eq!(tmp_posts.len(), 3);

    posts_iter = tmp_posts.iter();
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Valid post #4");
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Valid post #5");
    assert_eq!(posts_iter.next().unwrap().title.as_str(), "Locked post");
    assert!(posts_iter.next().is_none());
}

#[test]
fn read_all_posts_while_logged_in_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();

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

#[test]
fn create_post() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let p = send_create_post(
        &client,
        auth_token_header,
        new_post_title,
        new_post_content,
        "info",
        &[],
    );

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
}

#[test]
fn create_post_empty_title() {
    // clean database
    let client = init::clean_client();
    init::seed();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\", \"kind\": \"info\"}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn create_post_empty_content() {
    // clean database
    let client = init::clean_client();
    init::seed();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "New title";
    let new_post_content = "";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\", \"kind\": \"info\" }}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn create_post_simple_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let p = send_create_post(
        &client,
        auth_token_header,
        new_post_title,
        new_post_content,
        "info",
        &[],
    );

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
}

#[test]
fn create_post_unauthenticated_user() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";

    let post_json_data = format!(
        "{{\"title\": \"{}\",\"content\": \"{}\", \"kind\": \"info\"}}",
        new_post_title, new_post_content
    );

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .body(post_json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
fn create_post_missing_attribute() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";

    let post_json_data = format!("{{\"title\": \"{}\"}}", new_post_title);

    let req = client
        .post(POST_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(post_json_data);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
fn create_post_bad_json() {
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
    let response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);

    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        0
    );
}

#[test]
fn create_duplicate_post() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login_admin();

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_kind = "info";

    let p1 = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &[],
    );
    assert_eq!(p1.title, new_post_title);
    assert_eq!(p1.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p1.id).unwrap().is_some());
    assert_eq!(
        PostEntity::by_title(&conn, &new_post_title).unwrap().len(),
        1
    );

    let p2 = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &[],
    );

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
fn create_post_simple_user_with_tags() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_kind = "info";
    let new_post_tags = vec!["even", "odd"];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
    let tags = RelPostTagEntity::tags_by_post_id(&conn, &p.id)
        .unwrap()
        .iter()
        .map(|t| t.label.to_string())
        .collect::<Vec<String>>();

    for t in new_post_tags
        .iter()
        .map(|&tag| tag.to_string())
        .collect::<Vec<String>>()
    {
        assert!(tags.contains(&t));
    }
}

#[test]
fn create_post_simple_user_with_new_tags() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_kind = "info";
    let new_post_tags = vec!["frankyvincent", "lerestaurant"];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );

    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);

    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());
    let tags = RelPostTagEntity::tags_by_post_id(&conn, &p.id)
        .unwrap()
        .iter()
        .map(|t| t.label.to_string())
        .collect::<Vec<String>>();

    for t in new_post_tags
        .iter()
        .map(|tag| tag.to_string())
        .collect::<Vec<String>>()
    {
        assert!(tags.contains(&t));
    }
}

#[test]
fn read_a_post() {
    // clean database
    let client = init::clean_client();
    init::seed();

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
    let response = req.dispatch();

    //check the answer is a Bad request
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn read_a_post_soft_deleted() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let auth_token_header = init::login_admin();
    let existing_post_entity = init::get_post_entity(false, false, true);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn read_a_post_hidden_as_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();

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

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);
    let existing_post_entity = init::get_post_entity(false, true, false);

    // perform request
    let req = client
        .get(format!("{}/{}", POST_ROUTE, existing_post_entity.id))
        .header(auth_token_header);
    let response = req.dispatch();

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
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // update
    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
    let new_post_kind = "info";
    let new_post_tags = vec!["even", "odd"];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // lock the post
    let post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_lock(&conn).unwrap();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // hide the post
    let post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_visibility(&conn).unwrap();

    let updated_title = "updated title yo";
    let updated_content = "Les tests c'est quand même super non ?";
    let update_json_data = format!(
        "{{\"title\": \"{}\", \"content\":\"{}\", \"kind\": \"info\"}}",
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
fn delete_post_as_author() {
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
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(auth_token_header);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);
    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(updated_post.deleted);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn delete_post_as_admin() {
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
    let new_post_kind = "info";
    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &[],
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    let admin_auth_token_header = init::login_admin();
    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(admin_auth_token_header);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);
    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(updated_post.deleted);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn delete_post_wrong_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, false, false);

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(auth_token_header);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(!updated_post.deleted);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn delete_post_wrong_post_id() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let mut unexisting_id = 12;
    while PostEntity::by_id(&conn, &unexisting_id).unwrap().is_some() {
        unexisting_id += 1;
    }

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &unexisting_id))
        .header(init::login_admin());
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::BadRequest);
}

#[test]
fn delete_post_already_deleted() {
    // clean database
    let client = init::clean_client();
    init::seed();

    let p = init::get_post_entity(false, false, true);

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(init::login_admin());

    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::BadRequest);
}

#[test]
fn delete_post_locked_by_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(true, false, false);

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(init::login_admin());

    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);

    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(updated_post.deleted);
    assert!(updated_post.locked);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn delete_post_hidden_by_admin() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let p = init::get_post_entity(false, true, false);

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(init::login_admin());

    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Ok);

    let updated_post: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(updated_post.deleted);
    assert!(updated_post.hidden);
    assert_eq!(updated_post.id, p.id);
}

#[test]
fn delete_post_locked_by_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // lock the post
    let post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_lock(&conn).unwrap();

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(!not_updated.deleted);
    assert!(not_updated.locked);
    assert_eq!(not_updated.id, p.id);
}

#[test]
fn delete_post_hidden_by_author() {
    // clean database
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // post creation
    let new_post_title = "new post title";
    let new_post_content = "This is a new content for the post";
    let new_post_kind = "info";
    let new_post_tags = vec![];

    let p = send_create_post(
        &client,
        auth_token_header.clone(),
        new_post_title,
        new_post_content,
        new_post_kind,
        &new_post_tags,
    );
    assert_eq!(p.title, new_post_title);
    assert_eq!(p.content, new_post_content);
    assert!(PostEntity::by_id(&conn, &p.id).unwrap().is_some());

    // lock the post
    let post_entity = PostEntity::by_id(&conn, &p.id).unwrap().unwrap();
    post_entity.toggle_visibility(&conn).unwrap();

    let update_req = client
        .delete(format!("{}/{}", POST_ROUTE, &p.id))
        .header(ContentType::JSON)
        .header(auth_token_header);
    let update_response = update_req.dispatch();
    assert_eq!(update_response.status(), Status::Forbidden);

    let not_updated: Post = Post::from(PostEntity::by_id(&conn, &p.id).unwrap().unwrap());
    assert!(!not_updated.deleted);
    assert!(not_updated.hidden);
    assert_eq!(not_updated.id, p.id);
}
