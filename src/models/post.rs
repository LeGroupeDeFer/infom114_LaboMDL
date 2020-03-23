use std::ops::Deref; 

// use crate::database;
use crate::database::MyDbConn;
use crate::schema::posts;

use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub authorid: u32,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub nb_votes: u32,
}

impl Post {
    pub fn get_all_posts(conn: &MyDbConn) -> QueryResult<Vec<Post>> {
        posts::table.load::<Post>(conn.deref())
    }
}