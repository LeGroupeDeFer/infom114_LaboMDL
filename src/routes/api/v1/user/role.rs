//! # Role controller
//!
//! Group the creation, update and deletion of roles

use rocket_contrib::json::Json;

use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::guards::auth::Auth;

use crate::http::responders::{ok, ApiResult};
use crate::lib::EntityError;
use either::Either;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(assign, unassign)
}

/// Assign a role to a user
#[post("/api/v1/user/role", format = "json", data = "<data>")]
pub fn assign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResult<()> {
    let capability = "user:manage_role";

    // manage capability
    auth.check_capability(&*conn, &capability)?;

    let user_role_data = data.into_inner();

    let user = UserEntity::by_id(&*conn, &user_role_data.user_id)??;

    let role = RoleEntity::by_id(&*conn, &user_role_data.role_id)??;

    match RelUserRoleEntity::add_role_for_user(&*conn, &user, &role)? {
        Either::Right(_) => ok(),
        Either::Left(_) => Err(EntityError::Duplicate)?,
    }
}

/// Unassign a role from a user
#[delete("/api/v1/user/role", format = "json", data = "<data>")]
pub fn unassign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResult<()> {
    let capability = "user:manage_role";

    // manage capability
    auth.check_capability(&*conn, &capability)?;

    let user_role_data = data.into_inner();

    RelUserRoleEntity::get(&*conn, user_role_data.user_id, user_role_data.role_id)??
        .delete(&*conn)?;

    ok()
}
