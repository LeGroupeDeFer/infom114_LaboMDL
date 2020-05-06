//! # Comments controller
//!
//! Route related to the comment management

use crate::database::models::prelude::*;
use crate::database::DBConnection;

use crate::guards::{Auth, ForwardAuth, PostGuard};

use crate::http::responders::ApiResult;
use crate::lib::EntityError;

use rocket::Route;
use rocket_contrib::json::Json;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<Route> {
    routes!(
        get,
        create_comment,
    )
}

#[get("/api/v1/post/<_post_id>/comments", rank = 2)]
pub fn get(
    conn: DBConnection,
    auth: ForwardAuth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<Vec<Comment>> {
    let user = UserEntity::by_id(&*conn, &auth.deref().sub)??;

    Ok(Json(
        CommentEntity::by_post_id(
            &*conn,
            &post_guard.post().id,
            user.has_capability(&*conn, "comment:view_hidden"),
        )?
        .into_iter()
        .map(move |entity| Comment::from(entity))
        .collect::<Vec<Comment>>(),
    ))
}

#[post("/api/v1/post/<_post_id>/comment", format = "json", data = "<data>")]
fn create_comment(
    conn: DBConnection, 
    auth: Auth, 
    post_guard: PostGuard, 
    _post_id: u32, 
    data:Json<NewComment>
) -> ApiResult<Comment> {
    let comment_request = data.into_inner();

    if comment_request.content == "" {
        Err(EntityError::InvalidAttribute)?;
    }

    let new_comment = CommentMinima {
        post_id: post_guard.post().id,
        content: comment_request.content,
        author_id: auth.sub,
        parent_id: None,
    };

    let ce = CommentEntity::insert_either(&*conn, &new_comment)?;
    Ok(Json(Comment::from(ce)))
}