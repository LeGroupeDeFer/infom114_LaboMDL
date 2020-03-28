use crate::database::models::roles::forms::RoleData;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;

use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(create, update, delete)
}

#[post("/", format = "json", data = "<data>")]
pub fn create(conn: DBConnection, data: Json<RoleData>) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!("bite"))
}

#[put("/<role_id>", format = "json", data = "<data>")]
pub fn update(conn: DBConnection, role_id: u32, data: Json<RoleData>) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!("bite"))
}

#[delete("/<role_id>", format = "json", data = "<data>")]
pub fn delete(conn: DBConnection, role_id: u32, data: Json<RoleData>) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!("bite"))
}
