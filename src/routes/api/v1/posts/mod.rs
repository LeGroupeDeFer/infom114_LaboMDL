use crate::database::MyDbConn;
use crate::http::responders::api::ApiResponse;
use crate::models::post::Post;
use crate::schema::posts;

use super::forms::NewPost;

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

#[post("/posts", data = "<post_info>")]
fn create_post(post_info: Result<Json<NewPost>, JsonError>, conn: MyDbConn) -> ApiResponse {
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
fn get_all_posts(conn: MyDbConn) -> ApiResponse {
    // TODO: Get all related comments
    match Post::get_all_posts(&conn) {
        Ok(posts) => ApiResponse::new(Status::Ok, json!(posts)),
        Err(e) => ApiResponse::db_error(e),
    
    }
}