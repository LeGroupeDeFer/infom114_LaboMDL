use crate::database::models::roles::forms::RoleData;
use crate::database::models::roles::{
    capability::Capability,
    role::{Role, RoleMinima},
    role_capability::RelRoleCapability,
};
use crate::database::{DBConnection, Data};
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;

use rocket::http::Status;

use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(create, update, delete)
}

#[post("/api/v1/role", format = "json", data = "<data>")]
pub fn create(conn: DBConnection, data: Json<RoleData>) -> ApiResponse {
    // convert data into a usable type
    let role_data = data.into_inner();

    // create a new roleminima object
    let new_role = RoleMinima {
        name: role_data.name.into(),
        color: role_data.color.into(),
    };

    // insert the new role into database
    let role = match Role::insert_minima(&conn, &new_role) {
        Data::Existing(_) => {
            return ApiResponse::error(Status::Conflict, "A role with this name already exist")
        }
        Data::Inserted(r) => r,
    };

    // for this new role, add every given capabilities
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = Capability::from_name(&conn, &capability_data.name) {
            RelRoleCapability::add_capability_for_role(&conn, &role, &capability);
        } else {
            // TODO : front-end sent an unexisting capability
        }
    }

    ApiResponse::new(Status::Ok, json!({}))
}

#[put("/api/v1/role/<role_id>", format = "json", data = "<data>", rank = 3)]
pub fn update(conn: DBConnection, role_id: u32, data: Json<RoleData>) -> ApiResponse {
    let opt_role = Role::from_id(&conn, &role_id);

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
    if Role::from_name(&conn, &role_data.name).is_some() {
        return ApiResponse::error(Status::Conflict, "A role with this name already exist");
    }

    role.update(
        &conn,
        &RoleMinima {
            name: role_data.name.into(),
            color: role_data.color.into(),
        },
    );

    // reset capabilities
    role.clear_capabilities(&conn);

    // add every given capabilities
    for capability_data in role_data.capabilities.iter() {
        if let Some(capability) = Capability::from_name(&conn, &capability_data.name) {
            RelRoleCapability::add_capability_for_role(&conn, &role, &capability);
        } else {
            // TODO : front-end sent an unexisting capability
        }
    }

    ApiResponse::new(Status::Ok, json!({}))
}

#[delete("/api/v1/role/<role_id>")]
pub fn delete(conn: DBConnection, role_id: u32) -> ApiResponse {
    let opt_role = Role::from_id(&conn, &role_id);

    // assert that the role_id given exist
    if opt_role.is_none() {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted role does not exist",
        );
    }
    let role = opt_role.unwrap();
    // reset capabilities
    role.clear_capabilities(&conn);

    // delete role
    role.delete(&conn);

    ApiResponse::new(Status::Ok, json!({}))
}
