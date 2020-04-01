use crate::database::models::tag::Tag;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(post_tag, update_tag, delete_tag)
}

#[post("/api/v1/tag/<tag_label>", format = "json")]
pub fn post_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    //TODO    
    println!("Result : {}", tag_label);

    ApiResponse::new(
        Status::Ok,
        json!({
            "200" : "OK"
        }),
    )
}

#[put("/api/v1/tag/<tag_label>", format = "json")]
pub fn update_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    //TODO
    println!("Result : {}", tag_label);

    ApiResponse::new(
        Status::Ok,
        json!({
            "tag_put" : "todo"
        }),
    )
}

#[delete("/api/v1/tag/<tag_label>")]
pub fn delete_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    //TODO
    println!("Result : {}", tag_label);

    ApiResponse::new(
        Status::Ok,
        json!({
            "tag_delete" : "todo"
        }),
    )
}