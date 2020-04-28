//! # Role controller
//!
//! Group the creation, update and deletion of roles


use crate::guards::auth::Auth;
use crate::lib::consequence::*;
use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use rocket::http::Status;

use crate::database::models::role::forms::RoleData;
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
pub fn create(conn: DBConnection, auth: Auth, data: Json<RoleData>) -> ApiResponse {
    let capability = "role:manage";

    // manage capability

    if !auth.has_capability(&*conn, &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    // convert data into a usable type
    let role_data = data.into_inner();

    // prevent the creation of empty role name
    if &role_data.name == &"" {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The role name cannot be an empty string",
        );
    }

    // create a new roleminima object
    let new_role = RoleMinima {
        name: role_data.name.into(),
        color: role_data.color.into(),
    };

    // insert the new role into database
    let role = match RoleEntity::insert_new(&*conn, &new_role) {
        Err(Error::EntityError(EntityError::Duplicate)) =>
            return ApiResponse::error(Status::Conflict, "A role with this name already exist"),
        Ok(r) => r,
        _ => panic!("unreachable code reached")
    };

    // for this new role, add every given capability
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = CapabilityEntity::by_name(&*conn, &capability_data.name).unwrap() {
            RelRoleCapabilityEntity::add_capability_for_role(&*conn, &role, &capability);
        } else {
            // TODO : front-end sent an unexisting capability
        }
    }

    ApiResponse::new(Status::Ok, json!({}))
}

/// Update an existing role
///
/// When updating, the caller MUST specify each and every details, because
/// there is no smart mechanism implemented to perform a differencial update.
/// The old values are removed and the new values are inserted.
#[put("/api/v1/role/<role_id>", format = "json", data = "<data>", rank = 3)]
pub fn update(conn: DBConnection, auth: Auth, role_id: u32, data: Json<RoleData>) -> ApiResponse {
    let capability = "role:manage";

    // manage capability
    if !auth.has_capability(&*conn, &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let opt_role = RoleEntity::by_id(&*conn, &role_id).unwrap();
    // assert that the role_id given exist
    if opt_role.is_none() {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted role does not exist",
        );
    }
    let mut role = opt_role.unwrap();

    // assert that the new name is not already used
    let role_data = data.into_inner();
    if let Some(r) = RoleEntity::by_name(&*conn, &role_data.name).unwrap() {
        // we do not want to throw an error if the found role with the same
        // name is the one we are working on
        if r.id != role_id {
            return ApiResponse::error(Status::Conflict, "A role with this name already exist");
        }
    }

    role.name = role_data.name.into();
    role.color = role_data.color.into();
    role.update(&*conn);

    // reset capability
    role.clear_capabilities(&*conn);

    // add every given capability
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = CapabilityEntity::by_name(&*conn, &capability_data.name).unwrap() {
            RelRoleCapabilityEntity::add_capability_for_role(&*conn, &role, &capability);
        } else {
            // TODO : front-end sent an unexisting capability
        }
    }

    ApiResponse::new(Status::Ok, json!({}))
}

/// Delete an existing role
///
/// This will first remove every capability linked to this role
/// then it will remove the role.
#[delete("/api/v1/role/<role_id>")]
pub fn delete(conn: DBConnection, auth: Auth, role_id: u32) -> ApiResponse {
    let capability = "role:manage";

    // manage capability
    if !auth.has_capability(&*conn, &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let opt_role = RoleEntity::by_id(&*conn, &role_id).unwrap();

    // assert that the role_id given exist
    if opt_role.is_none() {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted role does not exist",
        );
    }
    let role = opt_role.unwrap();

    // reset capability
    role.clear_capabilities(&*conn);

    // delete role
    role.delete(&*conn);

    ApiResponse::new(Status::Ok, json!({}))
}
