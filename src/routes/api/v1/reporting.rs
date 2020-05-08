use crate::database::models::prelude::*;
use crate::database::DBConnection;

use crate::guards::Auth;
use crate::http::responders::ApiResult;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(get_users_report, get_tags_report)
    // get_posts_report
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

#[get("/api/v1/report/tags")]
pub fn get_tags_report(conn: DBConnection, auth: Auth) -> ApiResult<Vec<TagReport>> {
    auth.check_capability(&*conn, "report:view")?;

    let mut tab: Vec<TagReport> = Vec::new();
    for tag in TagEntity::all(&*conn)? {
        let posts = PostEntity::by_tag(&*conn, &tag.id)?;
        tab.push(TagReport {
            tag: tag.label.to_string(),
            info: posts
                .iter()
                .filter(|&p| p.kind == u8::from(PostKind::Info))
                .count() as u64,
            idea: posts
                .iter()
                .filter(|&p| p.kind == u8::from(PostKind::Idea))
                .count() as u64,
            poll: posts
                .iter()
                .filter(|&p| p.kind == u8::from(PostKind::Poll))
                .count() as u64,
        });
    }

    Ok(Json(tab))
}
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
