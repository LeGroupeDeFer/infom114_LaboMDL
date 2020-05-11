//! # Comments controller
//!
//! Route related to the comment management

use crate::database::models::prelude::*;
use crate::database::DBConnection;

use crate::guards::{Auth, ForwardAuth, PostGuard, CommentGuard};

use crate::http::responders::ApiResult;
use crate::lib::{AuthError, EntityError};

use rocket::Route;
use rocket_contrib::json::Json;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<Route> {
    routes!(
        get,
        create_comment,
        create_reply_to_comment,
        get_comment_authenticated,
        get_comment_unauthenticated, 
        updown_vote
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
    let can_view_hidden = auth.has_capability(&*conn, "comment:view_hidden");
    let can_edit_locked = auth.has_capability(&*conn, "comment:edit_locked");

    let comment_request = data.into_inner();
    if comment_request.content == "" {
        Err(EntityError::InvalidAttribute)?;
    }

    if post_guard.post().is_deleted() {
        Err(EntityError::InvalidID)?
    } 
    
    if (post_guard.post().is_locked() && !can_edit_locked)
        || (post_guard.post().is_hidden() && !can_view_hidden)
    {
        Err(AuthError::MissingCapability)?
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

#[post("/api/v1/comment/<_comment_id>", format = "json", data = "<data>")]
fn create_reply_to_comment(
    conn: DBConnection, 
    auth: Auth, 
    comment_guard: CommentGuard, 
    _comment_id: u32, 
    data:Json<NewComment>
) -> ApiResult<Comment> {
    let can_view_hidden = auth.has_capability(&*conn, "comment:view_hidden");
    let can_edit_locked = auth.has_capability(&*conn, "comment:edit_locked");

    let comment = comment_guard.comment();
    let post = PostEntity::by_id(&*conn, &comment.post_id).unwrap().unwrap();

    if comment.is_deleted() || post.is_deleted() {
        Err(EntityError::InvalidID)?
    } 
    
    if ((comment.is_locked() || post.is_locked()) && !can_edit_locked)
        || ((comment.is_hidden() || post.is_hidden()) && !can_view_hidden)
    {
        Err(AuthError::MissingCapability)?
    }

    let comment_request = data.into_inner();
    if comment_request.content == "" {
        Err(EntityError::InvalidAttribute)?
    }

    let new_comment = CommentMinima {
        post_id: comment_guard.comment().post_id,
        content: comment_request.content,
        author_id: auth.sub,
        parent_id: Some(comment_guard.comment().id),
    };

    let ce = CommentEntity::insert_either(&*conn, &new_comment)?;
    Ok(Json(Comment::from(ce)))
}


#[get("/api/v1/comment/<_comment_id>", rank = 1)]
fn get_comment_authenticated(
    conn: DBConnection,
    auth: ForwardAuth,
    comment_guard: CommentGuard, 
    _comment_id: u32
) -> ApiResult<Comment> {
    let comment = comment_guard.comment();
    let post = PostEntity::by_id(&*conn, &comment.post_id).unwrap().unwrap();

    let can_view_hidden = auth.deref().has_capability(&*conn, "comment:view_hidden");

    if comment.is_deleted() || post.is_deleted()
        || ((comment.is_hidden() || post.is_hidden()) && !can_view_hidden)
    {
        Err(EntityError::InvalidID)?
    }

    let mut c = Comment::from(comment_guard.comment_clone());
    c.set_user_info(&*conn, &auth.deref().sub);
    Ok(Json(c))
}

#[get("/api/v1/comment/<_comment_id>", rank = 2)] 
fn get_comment_unauthenticated(
    conn: DBConnection, 
    comment_guard: CommentGuard, 
    _comment_id: u32
) -> ApiResult<Comment> {
    let comment = comment_guard.comment();
    let post = PostEntity::by_id(&*conn, &comment.post_id).unwrap().unwrap();
    if comment.is_deleted() || comment.is_hidden() 
        || post.is_deleted() || post.is_hidden() 
    {
        Err(EntityError::InvalidID)?
    }

    Ok(Json(Comment::from(comment_guard.comment_clone())))
}

#[post("/api/v1/comment/<_comment_id>/vote", format = "json", data = "<data>")]
fn updown_vote(
    conn: DBConnection,
    auth: Auth,
    comment_guard: CommentGuard,
    _comment_id: u32,
    data: Json<ChangeVote>,
) -> ApiResult<Comment> {
    println!("Got in to the function!");
    let can_view_hidden = auth.has_capability(&*conn, "comment:view_hidden");
    let can_edit_locked = auth.has_capability(&*conn, "comment:edit_locked");

    let comment = comment_guard.comment();
    let post = PostEntity::by_id(&*conn, &comment.post_id).unwrap().unwrap();

    if comment.is_deleted() || post.is_deleted() {
        Err(EntityError::InvalidID)?
    } 
    
    if ((comment.is_locked() || post.is_locked()) && !can_edit_locked)
        || ((comment.is_hidden() || post.is_hidden()) && !can_view_hidden)
    {
        Err(AuthError::MissingCapability)?
    }

    let vote_request = data.into_inner();
    let mut comment_entity = comment_guard.comment_clone();
    comment_entity.upvote(&*conn, &auth.sub, vote_request.vote)?;

    let mut comment = Comment::from(comment_entity);
    comment.set_user_info(&*conn, &auth.sub);

    Ok(Json(comment))
}