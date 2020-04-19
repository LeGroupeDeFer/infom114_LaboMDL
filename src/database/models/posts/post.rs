use std::ops::Deref;

use diesel::MysqlConnection;
use crate::database::schema::posts;

use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub authorid: u32,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
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
    // get_all_posts :: (DBConnection) -> QueryResult<Vec<User>>
    pub fn get_all_posts(conn: &MysqlConnection) -> QueryResult<Vec<Post>> {
        posts::table.load::<Post>(conn.deref())
    }

    pub fn get_post_by_id(conn: &MysqlConnection, post_id: u32) -> Option<Post> {
        if let Ok(a_post) = posts::table.filter(posts::id.eq(post_id)).first(conn) {
            Some(a_post)
        } else {
            None
        }
    }

}