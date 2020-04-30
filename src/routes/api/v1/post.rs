use crate::database::models::prelude::*;
use crate::database::DBConnection;

use crate::guards::{Auth, ForwardAuth, PostGuard};
use crate::http::responders::{ApiResult, OK};
use crate::lib::{AuthError, Consequence, EntityError};

use rocket_contrib::json::Json;

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
        p.set_user_vote(&*conn, &auth.deref().sub);
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
) -> ApiResult<()> {
    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?;
    } else if false
        || (post_guard.post().is_locked() && !auth.has_capability(&*conn, "post:edit_locked"))
        || (post_guard.post().is_hidden() && !auth.has_capability(&*conn, "post:view_hidden"))
    {
        Err(AuthError::MissingCapability)?;
    }

    let vote_request = data.into_inner();

    post_guard
        .post()
        .upvote(&*conn, &auth.sub, vote_request.vote)?;

    OK()
}
