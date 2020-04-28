//! # Roles controller
//!
//! Route related to the fetching of roles

use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::guards::auth::Auth;

use rocket::http::Status;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(get)
}

/// Return every roles (and their capability) in database as a json array.
#[get("/api/v1/roles")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(RelRoleCapabilityEntity::all(&conn).unwrap()))
}