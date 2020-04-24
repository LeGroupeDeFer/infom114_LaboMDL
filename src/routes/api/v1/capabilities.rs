//! # Capabities controller
//!
//! Route related to the capability management

use crate::database::models::prelude::CapabilityEntity;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use std::ops::Deref;

use crate::guards::auth::Auth;

use rocket::http::Status;
use rocket::Route;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<Route> {
    routes!(get)
}

/// Read capabilities
///
/// The user needs to be authenticated, but this call do not require a special capability
/// All the capabilities stored in database are responded into the json format
#[get("/api/v1/capabilities")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(CapabilityEntity::all(conn.deref())))
}
