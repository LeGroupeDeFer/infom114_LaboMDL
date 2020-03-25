use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use crate::lib::extend_routes;
use rocket::http::Status;
mod auth;

pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = extend_routes("/auth", auth::collect());
    [&routes!(version)[..], &auth_routes[..]].concat()
}

#[get("/")]
pub fn version(_auth: Auth) -> ApiResponse {
    ApiResponse::new(
        Status::Ok,
        json!({
            "version": 1,
        }),
    )
}
