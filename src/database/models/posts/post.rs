use std::ops::Deref;

use crate::database::schema::posts;

use chrono::NaiveDateTime;

use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub authorid: u32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub nb_votes: u32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub authorid: u32,
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
}
