use crate::database::models::roles::RoleCapabilities;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get)
}

#[get("/api/v1/roles")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(RoleCapabilities::all(&conn)))
}
