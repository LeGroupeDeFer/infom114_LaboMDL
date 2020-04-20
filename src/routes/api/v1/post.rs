use crate::database::models::posts::{forms::NewPost, post::PostMinima};
use crate::database::models::prelude::{Post, User};
use crate::database::schema::posts;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(create_post, get_all_posts, get_post_by_id)
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
    let user = User::by_token(&*conn, &post_request.author_token);

    if let Some(author) = user {
        let new_post = PostMinima {
            title: post_request.title,
            content: post_request.content,
            authorid: author.id,
        };

        let insert_result = diesel::insert_into(posts::dsl::posts)
            .values(&new_post)
            .execute(&*conn);

        if insert_result.is_ok() {
            ApiResponse::new(
                Status::Ok,
                json!({
                    "msg":
                        &format!(
                            "Post '{}' of user '{}' inserted succesfully",
                            new_post.title, author.id
                        )
                }),
            )
        } else {
            // since we are sure that insert_result is a type Err, we can unwrap
            ApiResponse::db_error(insert_result.err().unwrap())
        }
    } else {
        ApiResponse::error(Status::Unauthorized, "Token not found!.")
    }
}

#[get("/posts")]
fn get_all_posts(conn: DBConnection) -> ApiResponse {
    // TODO: Get all related comments

    ApiResponse::new(Status::Ok, json!(Post::all(&conn)))
}

/// Get post by id (unauth)
#[get("/post/<post_id>")]
fn get_post_by_id(conn: DBConnection, post_id: String) -> ApiResponse {
    match post_id.parse::<u32>() {
        Ok(post_id) => match Post::by_id(&*conn, post_id) {
            Some(post) => ApiResponse::new(Status::Ok, json!(post)),
            None => ApiResponse::error(Status::NotFound, "Post not found"),
        },
        Err(_) => ApiResponse::error(Status::BadRequest, "Invalid id supplied"),
    }
}
