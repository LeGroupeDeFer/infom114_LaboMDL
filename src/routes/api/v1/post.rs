use crate::database::models::prelude::*;
use crate::database::{DBConnection, SortOrder};

use crate::guards::{Auth, ForwardAuth, PostGuard};
use crate::http::responders::{ApiResult, OK};
use crate::lib::{AuthError, Consequence, EntityError};

use rocket_contrib::json::Json;
use serde::export::TryFrom;

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        create_post,
        get_all_posts_authenticated,
        get_all_posts,
        get_post_authenticated,
        get_post,
        delete_post,
        update_post,
        updown_vote,
        toggle_visibility,
        toggle_lock,
        manage_post_report,
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["posts"]
}

/// Create a new post. Client data (title, content, auth_token)
/// + validate `auth_token`
/// + insert a new post into DB.
#[post("/api/v1/post", format = "json", data = "<data>")]
fn create_post(conn: DBConnection, auth: Auth, data: Json<NewPost>) -> ApiResult<Post> {
    let post_request = data.into_inner();

    if post_request.title == "" || post_request.content == "" {
        Err(EntityError::InvalidAttribute)?;
    }

    let new_post = PostMinima {
        title: post_request.title,
        content: post_request.content,
        author_id: auth.sub,
    };

    let p = PostEntity::insert_new(&*conn, &new_post)?;
    post_request.tags.map(|tags| -> Consequence<()> {
        for t in tags {
            let tag_entity = TagEntity::insert_either(&*conn, &TagMinima { label: t })?;
            p.add_tag(&*conn, &tag_entity.id)?;
        }
        Ok(())
    });

    Ok(Json(Post::from(p)))
}

// typo on tape is intentional : `type` is a rust reserved keyword
#[get(
    "/api/v1/posts?<tag>&<search>&<sort>&<tape>&<limit>&<offset>",
    rank = 1
)]
fn get_all_posts_authenticated(
    conn: DBConnection,
    auth: ForwardAuth,
    tag: Option<String>,
    search: Option<String>,
    sort: Option<String>,
    tape: Option<String>, // type
    limit: Option<u32>,
    offset: Option<u32>,
) -> ApiResult<Vec<Post>> {
    let mut sort_order: Option<SortOrder> = None;
    if let Some(value) = sort {
        sort_order = Some(SortOrder::try_from(value.as_ref())?)
    }

    let posts = Post::all(
        &*conn,
        auth.deref().has_capability(&*conn, "post:view_hidden"),
        tag,
        search,
        sort_order,
        tape,
        limit,
        offset,
    )?
    .into_iter()
    .map(move |mut p| {
        p.set_user_info(&*conn, &auth.deref().sub);
        p
    })
    .collect::<Vec<Post>>();
    Ok(Json(posts))
}
#[get(
    "/api/v1/posts?<tag>&<search>&<sort>&<tape>&<limit>&<offset>",
    rank = 2
)]
fn get_all_posts(
    conn: DBConnection,
    tag: Option<String>,
    search: Option<String>,
    sort: Option<String>,
    tape: Option<String>, // type
    limit: Option<u32>,
    offset: Option<u32>,
) -> ApiResult<Vec<Post>> {
    let mut sort_order: Option<SortOrder> = None;
    if let Some(value) = sort {
        sort_order = Some(SortOrder::try_from(value.as_ref())?)
    }
    Ok(Json(Post::all(
        &*conn, false, tag, search, sort_order, tape, limit, offset,
    )?))
}

#[get("/api/v1/post/<_post_id>", rank = 1)]
fn get_post_authenticated(
    conn: DBConnection,
    auth: ForwardAuth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<Post> {
    if !post_guard.post().is_deleted()
        && (!post_guard.post().is_hidden()
            || auth.deref().has_capability(&*conn, "post:view_hidden"))
    {
        let mut p = Post::from(post_guard.post_clone());
        p.set_user_info(&*conn, &auth.deref().sub);
        Ok(Json(p))
    } else {
        Err(EntityError::InvalidID)?
    }
}
#[get("/api/v1/post/<_post_id>", rank = 2)]
fn get_post(post_guard: PostGuard, _post_id: u32) -> ApiResult<Post> {
    if !post_guard.post().is_deleted() && !post_guard.post().is_hidden() {
        Ok(Json(Post::from(post_guard.post_clone())))
    } else {
        Err(EntityError::InvalidID)?
    }
}

/// Delete a post
#[delete("/api/v1/post/<_post_id>")]
fn delete_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<()> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().author_id != auth.sub && !auth.has_capability(&*conn, "post:delete"))
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    post_guard.post_clone().delete(&*conn)?;

    OK()
}

/// Update a post (title/content)
#[put("/api/v1/post/<_post_id>", format = "json", data = "<data>")]
fn update_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<NewPost>,
) -> ApiResult<()> {
    let a_post = data.into_inner();

    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().author_id != auth.sub && !auth.has_capability(&*conn, "post:update"))
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    let mut post = post_guard.post_clone();
    post.title = a_post.title;
    post.content = a_post.content;

    post.update(&*conn)?;

    OK()
}

#[post("/api/v1/post/<_post_id>/vote", format = "json", data = "<data>")]
fn updown_vote(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<ChangeVote>,
) -> ApiResult<Post> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    let vote_request = data.into_inner();

    let mut post_entity = post_guard.post_clone();
    post_entity.upvote(&*conn, &auth.sub, vote_request.vote)?;

    let mut post = Post::from(post_entity);
    post.set_user_info(&*conn, &auth.sub);

    Ok(Json(post))
}

#[post("/api/v1/post/<_post_id>/hide")]
fn toggle_visibility(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<()> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked") {
        Err(AuthError::MissingCapability)?;
    }
    auth.check_capability(&*conn, "post:hide")?;

    post_guard.post().toggle_visibility(&*conn)?;

    OK()
}

#[post("/api/v1/post/<_post_id>/lock")]
fn toggle_lock(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<()> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden") {
        Err(AuthError::MissingCapability)?;
    }
    auth.check_capability(&*conn, "post:lock")?;

    post_guard.post().toggle_lock(&*conn)?;

    OK()
}

#[post("/api/v1/post/<_post_id>/report", format = "json", data = "<data>")]
fn manage_post_report(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Option<Json<ReportData>>,
) -> ApiResult<()> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    match data {
        Some(json_data) => {
            let report_data = json_data.into_inner();

            post_guard
                .post()
                .report(&*conn, &auth.sub, report_data.reason)?;
        }
        None => {
            post_guard.post().remove_report(&*conn, &auth.sub)?;
        }
    }

    OK()
}
