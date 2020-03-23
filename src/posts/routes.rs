use crate::database::MyDbConn;
use crate::http::responders::api::ApiResponse;
// use crate::models::quick_response::Info;
// use crate::models::post::Post;
use crate::schema;

use super::forms::NewPost;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json,JsonError};

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        get_all_posts,
        create_post
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["posts"]
}

#[post("/posts/create_post", data = "<post_info>")]
fn create_post(post_info: Result<Json<NewPost>, JsonError>, conn: MyDbConn) -> ApiResponse {
    match post_info {
        Ok(info) => {
            let insert_result = diesel::insert_into(schema::posts::dsl::posts)
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
//     // get all posts from db with theirs associated tags, comments,
//     match Post::get_all_posts(&conn) {
//         Ok(result) => {
//             // store them in a JSON object
//             // Return them in the following ApiResponse object
            ApiResponse::new(
                Status::Ok,
                json!({"success": true})
            )
//         }
//         Err(e) => ApiResponse::db_error(e),
    
//     }
}