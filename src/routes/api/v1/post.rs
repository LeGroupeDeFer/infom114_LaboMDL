use crate::database::models::posts::forms::{ChangeVote, NewPost};
use crate::database::models::prelude::{PostEntity, PostMinima};
use crate::database::schema::posts;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::guards::auth::Auth;
use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use std::ops::Deref;

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
fn create_post(conn: DBConnection, auth: Auth, data: Json<NewPost>) -> ApiResponse {
    let post_request = data.into_inner();

    let new_post = PostMinima {
        title: post_request.title,
        content: post_request.content,
        author_id: auth.sub,
    };

    let insert_result = diesel::insert_into(posts::dsl::posts)
        .values(&new_post)
        .execute(conn.deref());

    if insert_result.is_ok() {
        ApiResponse::new(
            Status::Ok,
            json!({
                "msg":
                    &format!(
                        "Post '{}' of user '{}' inserted successfully",
                        new_post.title, auth.sub
                    )
            }),
        )
    } else {
        // since we are sure that insert_result is a type Err, we can unwrap
        ApiResponse::db_error(insert_result.err().unwrap())
    }
}

#[get("/api/posts")]
fn get_all_posts(conn: DBConnection) -> ApiResponse {
    // TODO: Get all related comments
    ApiResponse::new(Status::Ok, json!(PostEntity::all(&conn)))
}

/// Get post by id (unauth)
#[get("/post/<post_id>")]
fn get_post_by_id(conn: DBConnection, post_id: String) -> ApiResponse {
    match post_id.parse::<u32>() {
        Ok(post_id) => match PostEntity::by_id(conn.deref(), post_id) {
            Some(post) => ApiResponse::new(Status::Ok, json!(post)),
            None => ApiResponse::error(Status::NotFound, "Post not found"),
        },
        Err(_) => ApiResponse::error(Status::BadRequest, "Invalid post_id"),
    }
}

/// Delete a post
#[delete("/api/post/<post_id>")]
fn delete_post_by_id(conn: DBConnection, auth: Auth, post_id: u32) -> ApiResponse {
    let capability = "post:delete";

    // TODO : remove the unwrap() with the creation of a post guard
    let post = PostEntity::by_id(conn.deref(), post_id).unwrap();
    if !(auth.has_capability(conn.deref(), &capability) || post.author_id == auth.sub) {
        // TODO : return right management error
    }

    post.delete(conn.deref());

    ApiResponse::simple_success(Status::Ok)
}

/// Update a post (title/content)
#[put("/api/post/<post_id>", format = "json", data = "<data>")]
fn update_post_by_id(
    conn: DBConnection,
    auth: Auth,
    post_id: u32,
    data: Json<NewPost>,
) -> ApiResponse {
    let capability = "post:update";

    // TODO : do not use unwrap() but use a post guard instead
    let a_post = data.into_inner(); // TODO : remove me when guard is created
    let post = PostEntity::by_id(conn.deref(), post_id).unwrap();
    if !(auth.has_capability(conn.deref(), &capability) || post.author_id == auth.sub) {
        // TODO : return right management error
    }

    let minima = PostMinima {
        author_id: post.author_id,
        title: a_post.title,
        content: a_post.content,
    };

    match post.update(conn.deref(), &minima) {
        Some(_) => ApiResponse::new(
            Status::Ok,
            json!({
                "msg":
                    &format!(
                        "Update a post '{}' of user '{}' successfully!",
                        post_id, auth.sub
                    )
            }),
        ),
        None => ApiResponse::error(Status::NotFound, "TODO Server error"),
    }
}

#[post("/api/post/<post_id>/upvote", format = "json", data = "<data>")]
fn updown_vote(
    conn: DBConnection,
    auth: Auth,
    post_id: u32,
    data: Json<ChangeVote>,
) -> ApiResponse {
    let vote_request = data.into_inner();

    let post = PostEntity::by_id(conn.deref(), post_id).unwrap();

    match vote_request.vote {
        i if -1 <= i && i <= 1 => {
            let _new_score = post.upvote(conn.deref(), auth.sub, i);
            ApiResponse::success(
                Status::Ok,
                &format!(
                    "Change vote of post '{}' of user '{}' successfully!",
                    post_id, auth.sub
                ),
            )
        }
        _ => ApiResponse::error(
            Status::UnprocessableEntity,
            "The vote value has to be included in {-1, 0, 1}",
        ),
    }
}
