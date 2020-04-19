use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;
use crate::database::models::posts::post::Post;
use crate::database::models::posts::forms::NewPost;
use crate::database::schema::posts;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json,JsonError};

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        create_post,
        get_all_posts
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["posts"]
}

#[post("/api/post", format = "json", data = "<post_info>")]
fn create_post(conn: DBConnection, post_info: Result<Json<NewPost>, JsonError>) -> ApiResponse {
    match post_info {
        Ok(info) => {
            let insert_result = diesel::insert_into(posts::dsl::posts)
                .values(&*info)
                .execute(&*conn);

            match insert_result {
                Ok(result) => ApiResponse::new(Status::Ok, json!({"success": true, "title": info.title})),
                Err(e) => ApiResponse::db_error(e),
            }
            
        }
        Err(e) => ApiResponse::json_error(e),
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