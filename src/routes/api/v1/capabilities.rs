use crate::database::models::roles::capability::Capability;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;
use rocket::Route;

pub fn collect() -> Vec<Route> {
    routes!(get)
}

#[get("/")]
pub fn get(conn: DBConnection, _auth: Auth) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(Capability::all(&conn)))
}
