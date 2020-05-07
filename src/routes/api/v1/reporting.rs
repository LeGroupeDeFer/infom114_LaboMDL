use crate::database::models::prelude::*;
use crate::database::DBConnection;
//use crate::guards::auth::Auth;

use crate::guards::Auth;
use crate::http::responders::ApiResult;
use rocket::http::Status;
use rocket_contrib::json::Json;

//use crate::lib::EntityError;
//use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get_users_report)
    // routes!(get_users_report, get_tags_report, get_posts_report)
}

#[get("/api/v1/report/users")]
pub fn get_users_report(conn: DBConnection, auth: Auth) -> ApiResult<CountUserForm> {
    auth.check_capability(&*conn, "report:view")?;
    let response = CountUserForm {
        total: UserEntity::count_users(&*conn, false)?,
        active: UserEntity::count_users(&*conn, true)?,
    };
    Ok(Json(response))
}

// #[get("/api/v1/report/tags")]
// pub fn get_tags_report(_conn: DBConnection) -> ApiResponse {
//     //let tags = TagEntity::all(&*conn).unwrap();
//     ApiResponse::new(
//         Status::Ok,
//         json!({
//             "todo" : "TODO"
//         }),
//     )
// }
//
// #[get("/api/v1/report/posts")]
// pub fn get_posts_report(_conn: DBConnection) -> ApiResponse {
//     //let tags = TagEntity::all(&*conn).unwrap();
//     ApiResponse::new(
//         Status::Ok,
//         json!({
//             "todo" : "TODO"
//         }),
//     )
// }
