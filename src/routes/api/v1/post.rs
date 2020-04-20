use crate::database::models::posts::forms::ChangeVote;
use crate::database::models::posts::forms::NewPost;
use crate::database::models::posts::post::Post;
use crate::database::models::posts::post::PostMinima;
use crate::database::models::user::User;
use crate::database::schema::posts;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonError};

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        create_post,
        get_all_posts,
        get_post_by_id,
        delete_post_by_id,
        update_post_by_id,
        updown_vote,
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["posts"]
}

/// Create a new post. Client data (title, content, auth_token)
/// + validate `auth_token`
/// + insert a new post into DB.
#[post("/api/post", format = "json", data = "<data>")]
fn create_post(conn: DBConnection, data: Json<NewPost>) -> ApiResponse {
    let post_request = data.into_inner();
    let user_id = User::get_id_by_token(&conn, &post_request.author_token);

    if let Some(authorid) = user_id {
        let new_post = PostMinima {
            title: post_request.title,
            content: post_request.content,
            authorid: authorid,
        };

        let insert_result = diesel::insert_into(posts::dsl::posts)
            .values(&new_post)
            .execute(&*conn);

        match insert_result {
            Ok(_) => ApiResponse::new(
                Status::Ok,
                json!({
                    "msg":
                        &format!(
                            "Post '{}' of user '{}' inserted succesfully",
                            new_post.title, authorid
                        )
                }),
            ),
            Err(e) => ApiResponse::db_error(e),
        }
    } else {
        ApiResponse::error(Status::Unauthorized, "Token not found!.")
    }
}

#[get("/api/posts")]
fn get_all_posts(conn: DBConnection) -> ApiResponse {
    // TODO: Get all related comments
    match Post::get_all_posts(&conn) {
        Ok(posts) => ApiResponse::new(Status::Ok, json!(posts)),
        Err(e) => ApiResponse::db_error(e),
    }
}

/// Get post by post_id (unauth)
#[get("/api/post/<post_id>")]
fn get_post_by_id(conn: DBConnection, post_id: String) -> ApiResponse {
    match post_id.parse::<u32>() {
        Ok(post_id) => match Post::get_post_by_id(&conn, post_id) {
            Some(post) => ApiResponse::new(Status::Ok, json!(post)),
            None => ApiResponse::error(Status::NotFound, "Post not found"),
        },
        Err(_) => ApiResponse::error(Status::BadRequest, "Invalid post_id"),
    }
}

/// Delete a post by post_id (auth)
#[delete("/api/post/<post_id>/<author_token>")]
fn delete_post_by_id(conn: DBConnection, post_id: String, author_token: String) -> ApiResponse {
    fn do_soft_delete(
        conn: DBConnection,
        post_id: u32,
        post_author_id: u32,
        _: Option<i32>,
        _: Option<NewPost>,
    ) -> ApiResponse {
        ApiResponse::new(
            Status::Ok,
            json!({
                "msg":
                    &format!(
                        "Delete a post '{}' of user '{}' successfully!",
                        post_id, post_author_id
                    )
            }),
        )
    }
    process_request_with_validation(conn, post_id, author_token, None, None, do_soft_delete)
}

/// Update a post (title/content) by post_id (auth)
#[put("/api/post/<post_id>", format = "json", data = "<data>")]
fn update_post_by_id(conn: DBConnection, post_id: String, data: Json<NewPost>) -> ApiResponse {
    let a_post = data.into_inner();
    let author_token = a_post.author_token.to_string();

    fn do_update(
        conn: DBConnection,
        post_id: u32,
        post_author_id: u32,
        _: Option<i32>,
        post_data: Option<NewPost>,
    ) -> ApiResponse {
        let a_post = post_data.unwrap();
        match Post::update_post(&conn, post_id, a_post.title, a_post.content) {
            Some(_) => ApiResponse::new(
                Status::Ok,
                json!({
                    "msg":
                        &format!(
                            "Update a post '{}' of user '{}' successfully!",
                            post_id, post_author_id
                        )
                }),
            ),
            // TODO server error
            None => ApiResponse::error(Status::NotFound, "TODO Server error"),
        }
    }

    process_request_with_validation(conn, post_id, author_token, None, Some(a_post), do_update)
}

/// Delete a post by post_id (auth)
#[post("/api/post/<post_id>/upvote", format = "json", data = "<data>")]
fn updown_vote(conn: DBConnection, post_id: String, data: Json<ChangeVote>) -> ApiResponse {
    let vote_request = data.into_inner();
    let upvote = vote_request.upvote;
    let author_token = vote_request.author_token;

    fn do_updown_vote(
        conn: DBConnection,
        post_id: u32,
        post_author_id: u32,
        change_vote: Option<i32>,
        _: Option<NewPost>,
    ) -> ApiResponse {
        // FIXME nb_votes type from u32 to i32
        let n_change = change_vote.unwrap_or_default() as u32;
        match Post::upvote(&conn, post_id, n_change) {
            Some(new_nb_votes) => ApiResponse::new(
                Status::Ok,
                json!({
                    "msg":
                        &format!(
                            "Change vote of post '{}' of user '{}' successfully!",
                            post_id, post_author_id
                        ),
                    "nb_votes": new_nb_votes
                }),
            ),
            None => ApiResponse::error(Status::NotFound, "TODO Server error"),
        }
    }

    process_request_with_validation(
        conn,
        post_id,
        author_token,
        Some(upvote),
        None,
        do_updown_vote,
    )
}

/// Process client's request relating to post with param validation and authentication.
/// 1: check if `post_id` is valid (u32), if failed return 400 Bad Request
/// 2: check user authentication, if failed return 401 Unauthorized
/// 3: get `post_author_id` from `post_id`, if failed return custome DB error
/// 4: make sure the authorized user can delete/update his own post
///     or vote up/down other's post, if failed return 403 Forbidden.
/// TODO: add option to allow admin do the same thing
/// `client_request` is a function pointer to do delete/update
fn process_request_with_validation(
    conn: DBConnection,
    post_id: String,
    author_token: String,
    change_vote: Option<i32>,
    a_post: Option<NewPost>,
    client_request: fn(DBConnection, u32, u32, Option<i32>, Option<NewPost>) -> ApiResponse,
) -> ApiResponse {
    match post_id.parse::<u32>() {
        Ok(post_id) => {
            let user_id = User::get_id_by_token(&conn, &author_token);

            if let Some(client_author_id) = user_id {
                let post_author_id = Post::get_author_id_by_post_id(&conn, post_id);

                if let Some(post_author_id) = post_author_id {
                    let allowed = if change_vote.is_some() {
                        // up/down vote: can not up/down the vote of his own
                        post_author_id != client_author_id
                    } else {
                        // delete or update vote: can not modify post of other person
                        post_author_id == client_author_id
                    };
                    if allowed {
                        client_request(conn, post_id, post_author_id, change_vote, a_post)
                    } else {
                        ApiResponse::error(
                            Status::Forbidden,
                            &format!(
                                "User '{}' is not allowed to take action on post '{}' of user '{}'",
                                client_author_id, post_id, post_author_id
                            ),
                        )
                    }
                } else {
                    // TODO return server.
                    ApiResponse::error(
                        Status::BadRequest,
                        "Cannot get `post_author_id` from `post_id`",
                    )
                }
            } else {
                ApiResponse::error(Status::Unauthorized, "Token not found!.")
            }
        }
        Err(_) => ApiResponse::error(Status::BadRequest, "Invalid post_id"),
    }
}
