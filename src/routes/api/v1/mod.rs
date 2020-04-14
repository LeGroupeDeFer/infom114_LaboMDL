use crate::auth::Auth;
use crate::http::responders::ApiResult;
use crate::lib::extend_routes;
use rocket_contrib::json::Json;
mod auth;

pub fn collect() -> Vec<rocket::Route> {
    let auth_routes = extend_routes("/auth", auth::collect());
    [&routes!(version)[..], &auth_routes[..]].concat()
}

#[derive(Serialize, Deserialize)]
pub struct ApiVersion {
    version: u32
}

#[get("/")]
pub fn version(_auth: Auth) -> ApiResult<ApiVersion> {
    Ok(Json(ApiVersion { version: 1 }))
}
