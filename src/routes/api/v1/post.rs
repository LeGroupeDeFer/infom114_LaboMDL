use crate::database::models::prelude::*;
use crate::database::{DBConnection, SortOrder};

use crate::guards::{Auth, ForwardAuth, PostGuard};
use crate::http::{
    helpers::StringVector,
    responders::{ok, ApiResult},
};
use crate::lib::{AuthError, Consequence, EntityError};

use crate::database::models::post::{
    WatchEventData, WatchEventEntity, WatchEventKind, WatchEventMinima,
};
use rocket_contrib::json::Json;
use serde::export::TryFrom;
use std::convert::{AsRef, TryInto};

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
        watch_post,
        manage_post_report,
        get_poll_post_authenticated,
        get_poll_post,
        vote_poll_post,
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

    // prevent empty title & empty content
    if post_request.title == "" {
        return Err(EntityError::InvalidAttribute.into());
    }

    let post_kind = PostKind::try_from((&post_request).kind.clone())?;
    let new_post = PostMinima {
        title: post_request.title,
        content: post_request.content.unwrap_or("".into()),
        author_id: auth.sub,
        kind: (&post_kind).into(),
    };

    let p = PostEntity::insert_new(&*conn, &new_post)?;
    post_request.tags.map(|tags| -> Consequence<()> {
        for t in tags {
            let tag_entity = TagEntity::insert_either(&*conn, &TagMinima { label: t })?;
            p.add_tag(&*conn, &tag_entity.id)?;
        }
        Ok(())
    });

    match post_kind {
        PostKind::Info => {}
        PostKind::Idea => {}
        PostKind::Poll => match post_request.options {
            Some(options) => {
                PollAnswerEntity::bulk_insert(
                    &*conn,
                    &p.id,
                    options
                        .iter()
                        .map(AsRef::as_ref)
                        .collect::<Vec<&str>>()
                        .as_ref(),
                )?;
            }
            None => Err(EntityError::EmptyAttribute)?,
        },
        _ => {
            // stress the fact that the kind is not implemented
            unimplemented!();
        }
    }

    Ok(Json(Post::from(p)))
}

// typo on tape is intentional : `type` is a rust reserved keyword
#[get(
    "/api/v1/posts?<tags>&<keywords>&<order>&<kind>&<author>&<limit>&<offset>",
    rank = 1
)]
fn get_all_posts_authenticated(
    conn: DBConnection,
    auth: ForwardAuth,
    tags: StringVector,
    keywords: StringVector,
    order: Option<String>,
    kind: Option<String>,
    author: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> ApiResult<Vec<Post>> {
    let mut sort_order: Option<SortOrder> = None;
    if let Some(value) = order {
        sort_order = Some(SortOrder::try_from(value.as_ref())?)
    }

    let posts = Post::all(
        &*conn,
        auth.deref().has_capability(&*conn, "post:view_hidden"),
        (*tags).clone(),
        (*keywords).clone(),
        sort_order,
        kind,
        author,
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
    "/api/v1/posts?<tags>&<keywords>&<order>&<kind>&<author>&<limit>&<offset>",
    rank = 2
)]
fn get_all_posts(
    conn: DBConnection,
    tags: StringVector,
    keywords: StringVector,
    order: Option<String>,
    kind: Option<String>,
    author: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> ApiResult<Vec<Post>> {
    let mut sort_order: Option<SortOrder> = None;
    if let Some(value) = order {
        sort_order = Some(SortOrder::try_from(value.as_ref())?)
    }
    Ok(Json(Post::all(
        &*conn,
        false,
        (*tags).clone(),
        (*keywords).clone(),
        sort_order,
        kind,
        author,
        limit,
        offset,
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

    ok()
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

    // prevent empty title & empty content
    if a_post.title == "" {
        Err(EntityError::InvalidAttribute)?;
    }

    let mut post = post_guard.post_clone();
    post.title = a_post.title;
    post.content = a_post.content.unwrap_or("".into());

    post.update(&*conn)?;

    ok()
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
) -> ApiResult<Post> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked") {
        Err(AuthError::MissingCapability)?;
    }
    auth.check_capability(&*conn, "post:hide")?;

    let mut post = post_guard.post_clone();
    post.toggle_visibility(&*conn)?;

    let mut p = Post::from(post);
    p.set_user_info(&*conn, &auth.sub);
    Ok(Json(p))
}

#[post("/api/v1/post/<_post_id>/lock")]
fn toggle_lock(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<Post> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden") {
        Err(AuthError::MissingCapability)?;
    }
    auth.check_capability(&*conn, "post:lock")?;

    let mut post = post_guard.post_clone();
    post.toggle_lock(&*conn)?;

    let mut p = Post::from(post);
    p.set_user_info(&*conn, &auth.sub);
    Ok(Json(p))
}

#[post("/api/v1/post/<_post_id>/report", format = "json", data = "<data>")]
fn manage_post_report(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Option<Json<ReportData>>,
) -> ApiResult<Post> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    let post = post_guard.post_clone();
    match data {
        Some(json_data) => {
            let report_data = json_data.into_inner();
            post.report(&*conn, &auth.sub, report_data.reason)?;
        }
        None => {
            post.remove_report(&*conn, &auth.sub)?;
        }
    }

    let mut p = Post::from(post);
    p.set_user_info(&*conn, &auth.sub);
    Ok(Json(p))
}

#[post("/api/v1/post/<_post_id>/watch", format = "json", data = "<data>")]
fn watch_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<WatchEventData>,
) -> ApiResult<Post> {
    let payload = data.into_inner();
    let mut post = post_guard.post_clone();

    // Validation
    if (&post).is_deleted() {
        Err(EntityError::InvalidID)?;
    }

    let mut caps = vec!["post:watch"];
    if (&post).is_locked() {
        caps.push("post:edit_locked");
    }
    if (&post).is_hidden() {
        caps.push("post:view_hidden");
    }
    auth.check_capabilities(&*conn, caps)?;

    let _post_kind: PostKind = (&post).kind.try_into()?;

    let post_id = post.id;
    let mut past_events: Vec<WatchEventEntity> = WatchEventEntity::by_post_id(&*conn, &post_id)?;
    let last_event: Option<WatchEventKind> = WatchEventEntity::last(&mut past_events)
        .map(|le| WatchEventKind::try_from(le.event)) // Option<Result<WatchEventKind>>
        .transpose()?; // Result<Option<WatchEventKind>>
    let new_event = WatchEventKind::try_from(payload.code)?;

    WatchEventEntity::validate_transition((&last_event).as_ref(), &new_event)?;
    // End of validation

    let wem = WatchEventMinima {
        post_id,
        author_id: auth.sub,
        comment: payload.comment,
        event: new_event.into(),
    };

    // RIP wee :'(
    WatchEventEntity::insert_new(&*conn, &wem)?;

    // Update post rank
    post.watch_now();
    post.update(&*conn)?;

    let mut p = Post::from(post.clone());
    p.set_user_info(&*conn, &auth.sub);

    Ok(Json(p))
}

#[get("/api/v1/post/<_post_id>/poll", rank = 1)]
fn get_poll_post_authenticated(
    conn: DBConnection,
    auth: ForwardAuth,
    post_guard: PostGuard,
    _post_id: u32,
) -> ApiResult<PostPoll> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_hidden()
        && !auth.deref().has_capability(&*conn, "post:view_hidden")
    {
        Err(AuthError::MissingCapability)?;
    }

    let mut p = PostPoll::try_from(post_guard.post())?;
    p.set_user_info(&*conn, &auth.deref().sub)?;

    Ok(Json(p))
}

#[get("/api/v1/post/<_post_id>/poll", rank = 2)]
fn get_poll_post(post_guard: PostGuard, _post_id: u32) -> ApiResult<PostPoll> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if post_guard.post().is_hidden() {
        Err(AuthError::MissingHeader)?;
    }

    Ok(Json(PostPoll::try_from(post_guard.post())?))
}

#[post("/api/v1/post/<_post_id>/poll", data = "<data>")]
fn vote_poll_post(
    conn: DBConnection,
    auth: Auth,
    post_guard: PostGuard,
    _post_id: u32,
    data: Json<PollVote>,
) -> ApiResult<PostPoll> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?
    } else if post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden") {
        Err(AuthError::MissingCapability)?
    }

    let mut p = PostPoll::try_from(post_guard.post())?;
    let user_vote = data.into_inner();

    p.user_vote(&*conn, &auth.sub, &user_vote.answer_id)?;

    Ok(Json(p))
}
