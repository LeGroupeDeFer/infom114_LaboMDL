use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use crate::lib::extend_routes;
use rocket::http::Status;
mod auth;
mod capabilities;
mod role;
mod roles;

pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = extend_routes("/auth", auth::collect());
    let capabilities_routes = extend_routes("/capabilities", capabilities::collect());
    let roles_routes = extend_routes("/roles", roles::collect());
    let role_routes = extend_routes("/role", role::collect());
    [
        &routes!(version)[..],
        auth_routes.as_ref(),
        capabilities_routes.as_ref(),
        roles_routes.as_ref(),
        role_routes.as_ref(),
    ]
    .concat()
}

#[get("/", rank = 1)]
pub fn version(_auth: Auth) -> ApiResponse {
    ApiResponse::new(
        Status::Ok,
        json!({
            "version": 1,
        }),
    )
}
