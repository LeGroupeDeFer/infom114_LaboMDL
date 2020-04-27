//! # Role controller
//!
//! Group the creation, update and deletion of roles

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::guards::auth::Auth;
use crate::database::models::prelude::*;
use crate::database::{DBConnection, Data};

use crate::http::responders::api::ApiResponse;


/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(assign, unassign)
}


/// Assign a role to a user
#[post("/api/v1/user/role", format = "json", data = "<data>")]
pub fn assign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResponse {
    let capability = "user:manage_role";

    // manage capability
    if !auth.has_capability(&*conn, &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let user_role_data = data.into_inner();

    let user = match User::by_id(&*conn, &user_role_data.user_id).unwrap() {
        Some(u) => u,
        None => {
            return ApiResponse::error(
                Status::UnprocessableEntity,
                "The targeted user does not exist",
            )
        }
    };

    let role = match Role::by_id(&*conn, &user_role_data.role_id).unwrap() {
        Some(u) => u,
        None => {
            return ApiResponse::error(
                Status::UnprocessableEntity,
                "The targeted role does not exist",
            )
        }
    };

    match RelUserRole::add_role_for_user(&*conn, &user, &role).unwrap() {
        Data::Inserted(_) => ApiResponse::simple_success(Status::Ok),
        Data::Existing(_) => {
            ApiResponse::error(Status::Conflict, "This user do already have this role")
        }
        _ => panic!("unreachable code reached"),
    }
}

/// Unassign a role from a user
#[delete("/api/v1/user/role", format = "json", data = "<data>")]
pub fn unassign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResponse {
    let capability = "user:manage_role";

    // manage capability
    if !auth.has_capability(&*conn, &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let user_role_data = data.into_inner();


    let rel_user_role =
        match RelUserRole::get(&*conn, user_role_data.user_id, user_role_data.role_id).unwrap() {
            Some(u_r) => u_r,
            None => {
                return ApiResponse::error(
                    Status::UnprocessableEntity,
                    "This user do not have this role",
                )
            }
        };

    rel_user_role.delete(&*conn);

    ApiResponse::simple_success(Status::Ok)
}
