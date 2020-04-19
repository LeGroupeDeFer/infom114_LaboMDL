use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;
use crate::database::schema::posts;
use crate::database::models::posts::post::Post;
use crate::database::models::posts::post::PostMinima;
use crate::database::models::posts::forms::NewPost;
use crate::database::models::user::User;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json,JsonError};

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        create_post,
        get_all_posts,
        get_post_by_id
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
            authorid: authorid
        };

        let insert_result = diesel::insert_into(posts::dsl::posts)
            .values(&new_post)
            .execute(&*conn);

        match insert_result {
            Ok(result) => ApiResponse::new(
                Status::Ok,
                json!({
                    "msg": &format!("Post '{}' of user '{}' inserted succesfully", new_post.title, authorid)
                })
            ),
            Err(e) => ApiResponse::db_error(e),
        }
    } else {
        ApiResponse::error(
            Status::Unauthorized,
            "Token not found!."
        )
    }
}

#[get("/posts")]
fn get_all_posts(conn: DBConnection) -> ApiResponse {
    // TODO: Get all related comments
    match Post::get_all_posts(&conn) {
        Ok(posts) => ApiResponse::new(Status::Ok, json!(posts)),
        Err(e) => ApiResponse::db_error(e),
    }
}


/// Get post by id (unauth)
#[get("/post/<post_id>")]
fn get_post_by_id(conn: DBConnection, post_id: String) -> ApiResponse {
    match post_id.parse::<u32>() {
        Ok(post_id) => match Post::get_post_by_id(&conn, post_id) {
            Some(post) => ApiResponse::new(Status::Ok, json!(post)),
            None => ApiResponse::error(Status::NotFound, "Post not found")
        },
        Err(e) => ApiResponse::error(Status::BadRequest, "Invalid id supplied")
    }
}