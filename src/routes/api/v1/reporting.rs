//use crate::database::models::prelude::*;
use crate::database::DBConnection;
//use crate::guards::auth::Auth;

use crate::http::responders::{api::ApiResponse};
use rocket::http::Status;

//use crate::lib::EntityError;
//use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get_users_report, get_tags_report, get_posts_report)
}

#[get("/api/v1/report/users")]
pub fn get_users_report(_conn: DBConnection) -> ApiResponse {
    //let total = UserEntity::get_number_of_active_users(&*conn).unwrap();
    ApiResponse::new(
        Status::Ok,
        json!({
            "total" : "todo",
            "active" : "todo"
        }),
    )
}

#[get("/api/v1/report/tags")]
pub fn get_tags_report(_conn: DBConnection) -> ApiResponse {
    //let tags = TagEntity::all(&*conn).unwrap();
    ApiResponse::new(
        Status::Ok,
        json!({
            "todo" : "TODO"
        }),
    )
}

#[get("/api/v1/report/posts")]
pub fn get_posts_report(_conn: DBConnection) -> ApiResponse {
    //let tags = TagEntity::all(&*conn).unwrap();
    ApiResponse::new(
        Status::Ok,
        json!({
            "todo" : "TODO"
        }),
    )
}
