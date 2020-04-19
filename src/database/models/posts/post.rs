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

impl Post {
    // get_all_posts :: (DBConnection) -> QueryResult<Vec<User>>
    pub fn get_all_posts(conn: &MysqlConnection) -> QueryResult<Vec<Post>> {
        posts::table.load::<Post>(conn.deref())
    }
}