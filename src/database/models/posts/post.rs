use std::ops::Deref;

use crate::database::schema::posts;
// use crate::database::Data;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;
use diesel::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

impl Post {
    /// Get all posts
    pub fn all(conn: &MysqlConnection) -> Vec<Post> {
        posts::table.load::<Post>(conn.deref()).unwrap_or(vec![])
    }

    /// Get a post by its id
    pub fn by_id(conn: &MysqlConnection, post_id: u32) -> Option<Post> {
        if let Ok(a_post) = posts::table.filter(posts::id.eq(post_id)).first(conn) {
            Some(a_post)
        } else {
            None
        }
    }

    /// Get `author_id` from a `post_id`
    pub fn get_author_id_by_post_id(conn: &MysqlConnection, post_id: u32) -> Option<u32> {
        let author_id = posts::table
            .filter(posts::id.eq(post_id))
            .select(posts::author_id)
            .first(conn);

        if let Ok(id) = author_id {
            Some(id)
        } else {
            None
        }
    }

    /// Delete a post permanently (not used)
    pub fn permanent_delete_post(conn: &MysqlConnection, post_id: u32) -> Option<usize> {
        let post_id = diesel::delete(posts::table.filter(posts::id.eq(post_id))).execute(conn);
        if let Ok(id) = post_id {
            Some(id)
        } else {
            None
        }
    }

    /// Soft-delete a post, aka. change `deleted_at` column
    pub fn soft_delete_post(conn: &MysqlConnection, post_id: u32) {}

    /// Update a post
    pub fn update_post(
        conn: &MysqlConnection,
        post_id: u32,
        new_title: String,
        new_content: String,
    ) -> Option<usize> {
        let target = posts::table.filter(posts::id.eq(post_id));
        let update_res = diesel::update(target)
            .set((posts::title.eq(new_title), posts::content.eq(new_content)))
            .execute(conn);
        if let Ok(_) = update_res {
            Some(1)
        } else {
            None
        }
    }

    pub fn upvote(conn: &MysqlConnection, post_id: u32, change_vote: i32) -> Option<i32> {
        let target = posts::table.filter(posts::id.eq(post_id));
        let score: Result<i32, _> = target.select(posts::score).first(conn);

        if let Ok(old_score) = score {
            let new_score = old_score + change_vote;
            diesel::update(target)
                .set(posts::score.eq(new_score))
                .execute(conn)
                .unwrap();
            Some(new_score)
        } else {
            None
        }
    }
}
