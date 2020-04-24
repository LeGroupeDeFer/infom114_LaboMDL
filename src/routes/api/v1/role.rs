//! # Role controller
//!
//! Group the creation, update and deletion of roles

use crate::database::models::prelude::{
    CapabilityEntity, RelRoleCapabilityEntity, RoleEntity, RoleMinima,
};
use crate::database::{DBConnection, Data};
use crate::guards::auth::Auth;
use crate::http::responders::api::ApiResponse;

use std::ops::Deref;

use rocket::http::Status;

use crate::database::models::roles::forms::RoleData;
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
    if !auth.has_capability(conn.deref(), &capability) {
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
    let role = match RoleEntity::insert_minima(conn.deref(), &new_role) {
        Data::Existing(_) => {
            return ApiResponse::error(Status::Conflict, "A role with this name already exist")
        }
        Data::Inserted(r) => r,
        _ => panic!("unreachable code reched"),
    };

    // for this new role, add every given capabilities
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = CapabilityEntity::by_name(conn.deref(), &capability_data.name) {
            RelRoleCapabilityEntity::add_capability_for_role(conn.deref(), &role, &capability);
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
    if !auth.has_capability(conn.deref(), &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let opt_role = RoleEntity::by_id(conn.deref(), &role_id);

    // assert that the role_id given exist
    if opt_role.is_none() {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted role does not exist",
        );
    }
    let role = opt_role.unwrap();

    // assert that the new name is not already used
    let role_data = data.into_inner();
    if let Some(r) = RoleEntity::by_name(conn.deref(), &role_data.name) {
        // we do not want to throw an error if the found role with the same
        // name is the one we are working on
        if r.id != role_id {
            return ApiResponse::error(Status::Conflict, "A role with this name already exist");
        }
    }

    role.update(
        conn.deref(),
        &RoleMinima {
            name: role_data.name.into(),
            color: role_data.color.into(),
        },
    );

    // reset capabilities
    role.clear_capabilities(conn.deref());

    // add every given capabilities
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = CapabilityEntity::by_name(conn.deref(), &capability_data.name) {
            RelRoleCapabilityEntity::add_capability_for_role(conn.deref(), &role, &capability);
        } else {
            // TODO : front-end sent an unexisting capability
        }
    }

    ApiResponse::new(Status::Ok, json!({}))
}

/// Delete an existing role
///
/// This will first remove every capabilities linked to this role
/// then it will remove the role.
#[delete("/api/v1/role/<role_id>")]
pub fn delete(conn: DBConnection, auth: Auth, role_id: u32) -> ApiResponse {
    let capability = "role:manage";

    // manage capability
    if !auth.has_capability(conn.deref(), &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let opt_role = RoleEntity::by_id(conn.deref(), &role_id);

    // assert that the role_id given exist
    if opt_role.is_none() {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted role does not exist",
        );
    }
    let role = opt_role.unwrap();

    // reset capabilities
    role.clear_capabilities(conn.deref());

    // delete role
    role.delete(conn.deref());

    ApiResponse::new(Status::Ok, json!({}))
}
