use crate::guards::auth::Auth;
use crate::database::DBConnection;
use crate::database::models::prelude::*;
use crate::http::responders::api::ApiResponse;

use std::ops::Deref;

use rocket::http::Status;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(post_tag, update_tag, delete_tag)
}

#[post("/api/v1/tag/<tag_label>", format = "json")]
pub fn post_tag(conn: DBConnection, _auth: Auth, tag_label: String) -> ApiResponse {
    let new_tag = TagMinima { label: tag_label };

    //TODO Update the json containing the specifications, it is not correct (see error 400)
    TagEntity::insert_new(&*conn, &new_tag).unwrap();
    return ApiResponse::new(Status::Ok, json!({}));
}

#[put("/api/v1/tag/<tag_label>", format = "json", data = "<data>")]
pub fn update_tag(
    conn: DBConnection,
    auth: Auth,
    tag_label: String,
    data: Json<TagData>,
) -> ApiResponse {
    let capability = "tag:update";

    // manage capability
    if !auth.has_capability(conn.deref(), &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    let tag_data = data.into_inner();

    if let Some(mut tag) = TagEntity::by_label(conn.deref(), &tag_label) {
        tag.label = tag_data.label;
        tag.update(&*conn).unwrap();
        ApiResponse::new(Status::Ok, json!({}))
    } else {
        ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted tag does not exist",
        )
    }
}

#[delete("/api/v1/tag/<tag_label>")]
pub fn delete_tag(conn: DBConnection, auth: Auth, tag_label: String) -> ApiResponse {
    let capability = "tag:update";

    // manage capability
    if !auth.has_capability(conn.deref(), &capability) {
        return ApiResponse::error(
            Status::Forbidden,
            &format!("The user do not have the capability {}", capability),
        );
    }

    if let Some(tag) = TagEntity::by_label(conn.deref(), &tag_label) {
        tag.delete(conn.deref());
        ApiResponse::new(Status::Ok, json!({}))
    } else {
        ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted tag does not exist",
        )
    }
}
