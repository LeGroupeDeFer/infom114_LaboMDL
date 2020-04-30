use crate::database::models::prelude::*;
use crate::database::schema::posts;
use crate::database::DBConnection;
use crate::http::responders::api::ApiResponse;

use crate::guards::auth::Auth;
use crate::guards::post::PostGuard;
use crate::guards::ForwardAuth;
use crate::http::responders::{ApiResult, OK};
use crate::lib::AuthError;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::Request;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        create_post,
        get_all_posts_authenticated,
        get_all_posts,
        get_post,
        delete_post,
        update_post,
        updown_vote,
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["posts"]
}

/// Create a new post. Client data (title, content, auth_token)
/// + validate `auth_token`
/// + insert a new post into DB.
#[post("/api/post", format = "json", data = "<data>")]
fn create_post(conn: DBConnection, auth: Auth, data: Json<NewPost>) -> ApiResponse {
    let post_request = data.into_inner();

    let new_post = PostMinima {
        title: post_request.title,
        content: post_request.content,
        author_id: auth.sub,
    };

    let insert_result = diesel::insert_into(posts::dsl::posts)
        .values(&new_post)
        .execute(&*conn);

    if insert_result.is_ok() {
        ApiResponse::new(
            Status::Ok,
            json!({
                "msg":
                    &format!(
                        "Post '{}' of user '{}' inserted successfully",
                        new_post.title, auth.sub
                    )
            }),
        )
    } else {
        // since we are sure that insert_result is a type Err, we can unwrap
        ApiResponse::db_error(insert_result.err().unwrap())
    }
}

#[get("/api/v1/posts", rank = 1)]
fn get_all_posts_authenticated(conn: DBConnection, auth: ForwardAuth) -> ApiResult<Vec<Post>> {
    let posts = if auth.deref().has_capability(&*conn, "post:view_hidden") {
        Post::admin_all(&*conn)?
    } else {
        Post::all(&*conn)?
    }
    .drain(..)
    .map(|mut p| {
        p.set_user_vote(&*conn, &auth.deref().sub);
        p
    })
    .collect::<Vec<Post>>();
    Ok(Json(posts))
}

#[get("/api/v1/posts", rank = 2)]
fn get_all_posts(conn: DBConnection) -> ApiResult<Vec<Post>> {
    Ok(Json(Post::all(&*conn)?))
}

/// Get post by id (unauth)
#[get("/post/<_post_id>")]
fn get_post(post_guard: PostGuard, _post_id: u32) -> ApiResponse {
    ApiResponse::new(Status::Ok, json!(post_guard.post()))
}

/// Delete a post
#[delete("/api/post/<_post_id>")]
fn delete_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResponse {
    let capability = "post:delete";

    if !(auth.has_capability(&*conn, &capability) || post_guard.post().author_id == auth.sub) {
        // TODO : return right management error
    }

    post_guard.post_clone().delete(&*conn);

    ApiResponse::simple_success(Status::Ok)
}

/// Update a post (title/content)
#[put("/api/post/<_post_id>", format = "json", data = "<data>")]
fn update_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<NewPost>,
) -> ApiResult<()> {
    let capability = "post:update";
    let a_post = data.into_inner();

    if !(auth.has_capability(&*conn, &capability) || post_guard.post().author_id == auth.sub) {
        Err(AuthError::MissingCapability)?;
    }

    let mut post = post_guard.post_clone();
    post.title = a_post.title;
    post.content = a_post.content;

    post.update(&*conn)?;

    OK()
}

#[post("/api/post/<_post_id>/upvote", format = "json", data = "<data>")]
fn updown_vote(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<ChangeVote>,
) -> ApiResponse {
    let vote_request = data.into_inner();

    match vote_request.vote {
        i if -1 <= i && i <= 1 => {
            let _new_score = post_guard.post().upvote(&*conn, auth.sub, i);
            ApiResponse::success(
                Status::Ok,
                &format!(
                    "Change vote of post '{}' of user '{}' successfully!",
                    post_guard.post().id,
                    auth.sub
                ),
            )
        }
        _ => ApiResponse::error(
            Status::UnprocessableEntity,
            "The vote value has to be included in {-1, 0, 1}",
        ),
    }
}
