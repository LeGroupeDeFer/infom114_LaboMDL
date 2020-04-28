//! Comment crate
//!
//! Here will be stored everything that is related to the commentaries
//! The post linked to them, the user that writes them, the number of replies, ...
use chrono::NaiveDateTime;

use crate::database::schema::comments;
use crate::database::tables::comments_table as table;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "comments"]
pub struct CommentEntity {
    pub id: u32,
    pub post_id: u32,
    pub parent_id: Option<u32>,
    pub content: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub votes: u32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "comments"]
pub struct CommentMinima {
    pub post_id: u32,
    pub content: String,
    pub author_id: u32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub votes: u32,
    pub parent_id: u32,
}

impl CommentEntity {
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    pub fn by_id(conn: &MysqlConnection, id: u32) -> Option<Self> {
        table.find(id).first(conn).ok()
    }

    pub fn by_post(conn: &MysqlConnection, post_id: u32) -> Vec<Self> {
        table
            .filter(comments::post_id.eq(post_id))
            .load(conn)
            .unwrap_or(vec![])
    }
}
