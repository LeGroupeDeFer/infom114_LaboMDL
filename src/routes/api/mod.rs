use crate::auth::forms::{ActivationData, LoginData, RegisterData};
use crate::auth::Auth;
use crate::conf;
use crate::database::models::roles::forms::{RoleData, UserRoleData};
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;
use rocket_contrib::json::Json;

use rocket;

mod v1;

pub fn collect() -> Vec<rocket::Route> {
    [
        &v1::collect()[..],
        &routes!(
            auth_register,
            auth_login,
            auth_activate,
            capabilities_get,
            role_create,
            role_update,
            role_delete,
            roles_get,
            user_role_assign,
            user_role_unassign
        )[..],
    ]
    .concat()
}

/*************************** AUTH ********************************************/

#[post("/api/auth/register", format = "json", data = "<data>")]
pub fn auth_register(conn: DBConnection, data: Json<RegisterData>) -> ApiResponse {
    v1::auth::register(conn, data)
}

#[post("/api/auth/login", format = "json", data = "<data>")]
pub fn auth_login(conn: DBConnection, state: conf::State, data: Json<LoginData>) -> ApiResponse {
    v1::auth::login(conn, state, data)
}

#[post("/api/auth/activate", format = "json", data = "<data>")]
pub fn auth_activate(conn: DBConnection, data: Json<ActivationData>) -> ApiResponse {
    v1::auth::activate(conn, data)
}

/*************************** ROLE MANAGEMENT *********************************/

#[get("/api/capabilities")]
pub fn capabilities_get(conn: DBConnection, auth: Auth) -> ApiResponse {
    v1::capabilities::get(conn, auth)
}

#[post("/api/role", format = "json", data = "<data>")]
pub fn role_create(conn: DBConnection, auth: Auth, data: Json<RoleData>) -> ApiResponse {
    v1::role::create(conn, auth, data)
}

#[put("/api/role/<role_id>", format = "json", data = "<data>", rank = 3)]
pub fn role_update(
    conn: DBConnection,
    auth: Auth,
    role_id: u32,
    data: Json<RoleData>,
) -> ApiResponse {
    v1::role::update(conn, auth, role_id, data)
}

#[delete("/api/role/<role_id>")]
pub fn role_delete(conn: DBConnection, auth: Auth, role_id: u32) -> ApiResponse {
    v1::role::delete(conn, auth, role_id)
}

#[get("/api/roles")]
pub fn roles_get(conn: DBConnection, auth: Auth) -> ApiResponse {
    v1::roles::get(conn, auth)
}

#[post("/api/user/role", format = "json", data = "<data>")]
pub fn user_role_assign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResponse {
    v1::user::role::assign(conn, auth, data)
}

#[delete("/api/user/role", format = "json", data = "<data>")]
pub fn user_role_unassign(conn: DBConnection, auth: Auth, data: Json<UserRoleData>) -> ApiResponse {
    v1::user::role::unassign(conn, auth, data)
}
