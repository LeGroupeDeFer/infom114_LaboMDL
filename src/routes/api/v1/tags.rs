use crate::database::models::tags::tag::Tag;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get)
}

#[get("/api/v1/tags")]
pub fn get(conn: DBConnection) -> ApiResponse {
    //TODO Update the json containing the specification of this api
    //TODO Do not send id information
    let tags = Tag::all(&conn);
    ApiResponse::new(Status::Ok, json!({ "tags": tags }))
}
