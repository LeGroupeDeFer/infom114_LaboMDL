use crate::database::models::roles::RoleCapabilities;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get)
}

#[get("/")]
pub fn get(conn: DBConnection) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(RoleCapabilities::all(&conn)))
}
