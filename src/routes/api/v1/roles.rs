//! # Roles controller
//!
//! Route related to the fetching of roles

use crate::database::models::roles::RoleCapabilities;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(get)
}

/// Return every roles (and their capabilities) in database as a json array.
#[get("/api/v1/roles")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(RoleCapabilities::all(&conn).unwrap()))
}
