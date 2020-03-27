use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use crate::lib::extend_routes;
use rocket::http::Status;
mod auth;
mod tag;

pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = extend_routes("/auth", auth::collect());
    let tag_routes = extend_routes("/tag", tag::collect());
    [&routes!(version)[..], &auth_routes[..], &tag_routes[..]].concat()
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
