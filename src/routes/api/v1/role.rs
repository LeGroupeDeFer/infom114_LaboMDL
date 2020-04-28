//! # Role controller
//!
//! Group the creation, update and deletion of roles

use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::guards::auth::Auth;
use crate::http::responders::api::ApiResponse;
use crate::lib::consequence::*;

use rocket::http::Status;

use crate::database::models::role::forms::RoleData;
use crate::http::responders::{ApiResult, OK};
use rocket_contrib::json::Json;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(create, update, delete)
}

/// Create a new role
///
/// This new role is the json serialization of the `RoleData` type
/// The name cannot be empty
#[post("/api/v1/role", format = "json", data = "<data>")]
pub fn create(conn: DBConnection, auth: Auth, data: Json<RoleData>) -> ApiResult<()> {
    // manage capability
    let capability = "role:manage";
    auth.check_capability(&*conn, &capability)?;

    // convert data into a usable type
    let role_data = data.into_inner();

    // create a new role minima object
    let new_role = RoleMinima {
        name: role_data.name.into(),
        color: role_data.color.into(),
    };

    // insert the new role into database
    let role = RoleEntity::insert_new(&*conn, &new_role)?;

    role.add_capabilities(&*conn, &role_data.capabilities)?;

    OK()
}

/// Update an existing role
///
/// When updating, the caller MUST specify each and every details, because
/// there is no smart mechanism implemented to perform a differential update.
/// The old values are removed and the new values are inserted.
#[put("/api/v1/role/<role_id>", format = "json", data = "<data>", rank = 3)]
pub fn update(conn: DBConnection, auth: Auth, role_id: u32, data: Json<RoleData>) -> ApiResult<()> {
    // manage capability
    let capability = "role:manage";
    auth.check_capability(&*conn, &capability)?;

    let mut role = RoleEntity::by_id(&*conn, &role_id)??;

    // assert that the new name is not already used
    let role_data = data.into_inner();
    if let Some(r) = RoleEntity::by_name(&*conn, &role_data.name)? {
        // we do not want to throw an error if the found role with the same
        // name is the one we are working on
        if r.id != role_id {
            Err(EntityError::Duplicate)?;
        }
    }

    role.name = role_data.name.into();
    role.color = role_data.color.into();
    role.update(&*conn)?;

    // reset capability
    role.clear_capabilities(&*conn)?;

    // add every given capability
    role.add_capabilities(&*conn, &role_data.capabilities)?;
    OK()
}

/// Delete an existing role
///
/// This will first remove every capability linked to this role
/// then it will remove the role.
#[delete("/api/v1/role/<role_id>")]
pub fn delete(conn: DBConnection, auth: Auth, role_id: u32) -> ApiResult<()> {
    // manage capability
    let capability = "role:manage";

    auth.check_capability(&*conn, &capability)?;

    let role = RoleEntity::by_id(&*conn, &role_id)??;

    // reset capability
    role.clear_capabilities(&*conn)?;

    // delete role
    role.delete(&*conn)?;

    OK()
}
