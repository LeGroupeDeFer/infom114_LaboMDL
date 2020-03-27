use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;
use crate::database::models::tag::Tag;

use rocket;
use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get_tags, post_tag, delete_tag, update_tag)
}

#[get("/")]
fn get_tags(conn: DBConnection) -> ApiResponse {
    let tags = Tag::all(&conn);
    ApiResponse::new(
        Status::Ok,
        json!({
            "tags" : tags
        }),
    )
}

#[post("/<tag_label>", format = "application/json")]
fn post_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    //TODO    
    let existence = Tag::available_label(&conn, &tag_label);
    println!("Result : {:?}", existence);

    ApiResponse::new(
        Status::Ok,
        json!({
            "200" : "OK"
        }),
    )
}

#[put("/", format = "application/json")]
fn update_tag(conn: DBConnection) -> ApiResponse {
    //TODO
    let result = Tag::all(&conn);
    println!("Result : {:?}", result);

    ApiResponse::new(
        Status::Ok,
        json!({
            "tag_put" : "todo"
        }),
    )
}

#[delete("/")]
fn delete_tag(conn: DBConnection) -> ApiResponse {
    //TODO
    let result = Tag::all(&conn);
    println!("Result : {:?}", result);

    ApiResponse::new(
        Status::Ok,
        json!({
            "tag_delete" : "todo"
        }),
    )
}