//! V1 controller
//!
//! The purpose of this module is to group every routes implemented on their
//! first version.
//!
//! Every child module SHOULD implement his own `collect` function and those
//! functions SHOULD be called on the `v1::collectt` function to group every
//! routes in each and every child module.

use crate::guards::auth::Auth;
use crate::http::responders::ApiResult;
use rocket_contrib::json::Json;

pub mod auth;
pub mod capabilities;
pub mod post;
pub mod role;
pub mod roles;
pub mod tag;
pub mod tags;
pub mod user;
pub mod users;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = auth::collect();
    let capabilities_routes = capabilities::collect();
    let roles_routes = roles::collect();
    let role_routes = role::collect();
    let tag_routes = tag::collect();
    let tags_routes = tags::collect();
    let user_routes = user::collect();
    let users_routes = users::collect();
    let post_routes = post::collect();
    [
        &routes!(version)[..],
        auth_routes.as_ref(),
        capabilities_routes.as_ref(),
        roles_routes.as_ref(),
        role_routes.as_ref(),
        tags_routes.as_ref(),
        tag_routes.as_ref(),
        user_routes.as_ref(),
        users_routes.as_ref(),
        post_routes.as_ref(),
    ]
    .concat()
}

#[derive(Serialize, Deserialize)]
pub struct ApiVersion {
    version: u32,
}

#[get("/api/v1")]
pub fn version(_auth: Auth) -> ApiResult<ApiVersion> {
    Ok(Json(ApiVersion { version: 1 }))
}
