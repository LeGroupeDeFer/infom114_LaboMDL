//! # User controller mod
//!
//! Here are grouped every routes and controller that are used in user
//! management.

use crate::database::models::prelude::{Post, PostEntity};
use crate::database::DBConnection;
use crate::guards::{Auth, UserGuard};
use crate::http::responders::ApiResult;
use rocket_contrib::json::Json;

pub mod role;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    [role::collect().as_ref(), &routes!(get_user_posts)[..]].concat()
}

#[get("/api/v1/user/<_user_id>/posts")]
pub fn get_user_posts(
    conn: DBConnection,
    auth: Auth,
    user_guard: UserGuard,
    _user_id: u32,
) -> ApiResult<Vec<Post>> {
    Ok(Json(
        PostEntity::by_author_id(&*conn, &user_guard.user().id)?
            .into_iter()
            .map(move |entity| {
                let mut p = Post::from(entity);
                p.set_user_info(&*conn, &auth.sub);
                p
            })
            .collect::<Vec<Post>>(),
    ))
}
