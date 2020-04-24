use crate::database::models::prelude::{TagEntity, TagMinima};
use crate::database::models::tags::forms::TagData;
use crate::database::DBConnection;
use crate::database::Data;
use crate::guards::auth::Auth;
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
    match TagEntity::insert(conn.deref(), &new_tag) {
        Data::Existing(_) => {
            return ApiResponse::error(Status::Conflict, "A tag with that name already exists")
        }
        Data::Inserted(_) => return ApiResponse::new(Status::Ok, json!({})),
        _ => {
            //This will never occur... but required by rust
            return ApiResponse::new(Status::Ok, json!({}));
        }
    };
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

    if let Some(tag) = TagEntity::by_label(conn.deref(), &tag_label) {
        match tag.update(conn.deref(), &tag_data.label) {
            Data::Updated(_) => return ApiResponse::new(Status::Ok, json!({})),
            Data::Existing(_) => {
                return ApiResponse::error(Status::Conflict, "A tag with this name already exists")
            }
            _ => {
                //This will never occur... but required by rust
                panic!("unreacheable code reached");
            }
        }
    } else {
        return ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted tag does not exist",
        );
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
