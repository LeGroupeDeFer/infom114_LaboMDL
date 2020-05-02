//! # Capabities controller
//!
//! Route related to the capability management

use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::guards::auth::Auth;

use rocket::http::Status;
use rocket::Route;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<Route> {
    routes!(get)
}

/// Read capability
///
/// The user needs to be authenticated, but this call do not require a special capability
/// All the capability stored in database are responded into the json format
#[get("/api/v1/capabilities")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(CapabilityEntity::all(&conn).unwrap()))
}
