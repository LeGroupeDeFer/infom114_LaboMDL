//! # Capabities controller
//!
//! Route related to the capability management

use crate::database::models::prelude::*;
use crate::database::DBConnection;

use crate::guards::auth::Auth;

use crate::http::responders::ApiResult;
use rocket::Route;
use rocket_contrib::json::Json;

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
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResult<Vec<Capability>> {
    Ok(Json(
        CapabilityEntity::all(&*conn)?
            .into_iter()
            .map(move |capability_entity| Capability::from(capability_entity))
            .collect::<Vec<Capability>>(),
    ))
}