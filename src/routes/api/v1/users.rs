use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::guards::Auth;
use crate::http::responders::ApiResult;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(get_all_users)
}

#[get("/api/users")]
pub fn get_all_users(conn: DBConnection, auth: Auth) -> ApiResult<Vec<User>> {
    let capability = "users:view";

    auth.check_capability(&*conn, &capability)?;

    Ok(UserEntity::all(&*conn)?
        .drain(..)
        .map(|user_entity| User::from(user_entity))
        .collect())
}
