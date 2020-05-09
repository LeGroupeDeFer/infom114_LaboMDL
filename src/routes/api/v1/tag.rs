use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::guards::auth::Auth;

use crate::http::responders::{ok, ApiResult};
use crate::lib::EntityError;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(post_tag, update_tag, delete_tag)
}

#[post("/api/v1/tag/<tag_label>")]
pub fn post_tag(conn: DBConnection, auth: Auth, tag_label: String) -> ApiResult<()> {
    auth.check_capability(&*conn, "tag:create")?;

    let new_tag = TagMinima { label: tag_label };

    TagEntity::insert_new(&*conn, &new_tag)?;
    ok()
}

#[put("/api/v1/tag/<tag_label>", format = "json", data = "<data>")]
pub fn update_tag(
    conn: DBConnection,
    auth: Auth,
    tag_label: String,
    data: Json<TagData>,
) -> ApiResult<()> {
    let capability = "tag:update";

    // manage capability
    auth.check_capability(&*conn, &capability)?;

    let tag_data = data.into_inner();

    if let Some(mut tag) = TagEntity::by_label(&*conn, &tag_label)? {
        tag.label = tag_data.label;
        tag.update(&*conn)?;
    } else {
        Err(EntityError::InvalidID)?;
    }

    ok()
}

#[delete("/api/v1/tag/<tag_label>")]
pub fn delete_tag(conn: DBConnection, auth: Auth, tag_label: String) -> ApiResult<()> {
    let capability = "tag:delete";

    // manage capability
    auth.check_capability(&*conn, &capability)?;

    match TagEntity::by_label(&*conn, &tag_label)? {
        Some(tag) => tag.delete(&*conn)?,
        None => Err(EntityError::InvalidID)?,
    }

    ok()
}
