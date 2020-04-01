use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use rocket::http::Status;

pub mod auth;
pub mod capabilities;
pub mod role;
pub mod roles;

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

#[get("/api/v1", rank = 1)]
pub fn version(_auth: Auth) -> ApiResponse {
    ApiResponse::new(
        Status::Ok,
        json!({
            "version": 1,
        }),
    )
}
