//! V1 controller
//!
//! The purpose of this module is to group every routes implemented on their
//! first version.
//!
//! Every child module SHOULD implement his own `collect` function and those
//! functions SHOULD be called on the `v1::collectt` function to group every
//! routes in each and every child module.

use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use rocket::http::Status;

pub mod auth;
pub mod capabilities;
pub mod role;
pub mod roles;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = auth::collect();
    let capabilities_routes = capabilities::collect();
    let roles_routes = roles::collect();
    let role_routes = role::collect();
    [
        &routes!(version)[..],
        auth_routes.as_ref(),
        capabilities_routes.as_ref(),
        roles_routes.as_ref(),
        role_routes.as_ref(),
    ]
    .concat()
}

/// Return the version of this module
/// why though ?
#[get("/api/v1", rank = 1)]
pub fn version(_auth: Auth) -> ApiResponse {
    ApiResponse::new(
        Status::Ok,
        json!({
            "version": 1,
        }),
    )
}
