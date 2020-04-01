use crate::auth::Auth;
use crate::http::responders::api::ApiResponse;
use rocket::http::Status;

pub mod auth;
pub mod tags;
pub mod tag;

pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = auth::collect();
    let tags_routes = tags::collect();
    let tag_routes = tag::collect();
    [
        &routes!(version)[..],
        auth_routes.as_ref(),
        tags_routes.as_ref(),
        tag_routes.as_ref(),
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
