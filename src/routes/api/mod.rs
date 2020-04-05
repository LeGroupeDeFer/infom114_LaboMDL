use crate::auth::forms::{ActivationData, LoginData, RegisterData};
use crate::conf;

use crate::database::DBConnection;
use crate::database::models::tags::forms::TagData;

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
            tags_get,
            tag_post,
            tag_update,
            tag_delete
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

/**************************** TAG MANAGEMENT *********************************/
#[get("/api/tags")]
pub fn tags_get(conn: DBConnection) -> ApiResponse {
    v1::tags::get(conn)
}

#[post("/api/tag/<tag_label>")]
pub fn tag_post(conn: DBConnection, tag_label: String) -> ApiResponse {
    v1::tag::post_tag(conn, tag_label)
}

#[put("/api/tag/<tag_label>", data="<data>")]
pub fn tag_update(conn: DBConnection, tag_label: String, data: Json<TagData>) -> ApiResponse {
    v1::tag::update_tag(conn, tag_label, data)
}

#[delete("/api/tag/<tag_label>")]
pub fn tag_delete(conn: DBConnection, tag_label: String) -> ApiResponse {
    v1::tag::delete_tag(conn, tag_label)
}
