use crate::database::models::tags::forms::TagData;
use crate::database::Data;

use crate::database::models::tags::tag::{Tag, TagMinima};
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use rocket_contrib::json::Json;

use rocket::http::Status;

pub fn collect() -> Vec<rocket::Route> {
    routes!(post_tag, update_tag, delete_tag)
}

#[post("/api/v1/tag/<tag_label>", format = "json")]
pub fn post_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    let new_tag = TagMinima { label: tag_label };

    //TODO Update the json containing the specifications, it is not correct (see error 400)
    match Tag::insert(&*conn, &new_tag) {
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
pub fn update_tag(conn: DBConnection, tag_label: String, data: Json<TagData>) -> ApiResponse {
    let tag_data = data.into_inner();

    if let Some(tag) = Tag::by_label(&conn, &tag_label) {
        match tag.update(&conn, &tag_data.label) {
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
pub fn delete_tag(conn: DBConnection, tag_label: String) -> ApiResponse {
    if let Some(tag) = Tag::by_label(&conn, &tag_label) {
        tag.delete(&conn);
        ApiResponse::new(Status::Ok, json!({}))
    } else {
        ApiResponse::error(
            Status::UnprocessableEntity,
            "The targeted tag does not exist",
        )
    }
}
