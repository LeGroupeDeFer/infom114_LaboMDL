//! Comment crate
//!
//! Here will be stored everything that is related to the commentaries
//! The post linked to them, the user that writes them, the number of replies, ...
use chrono::NaiveDateTime;

use crate::database::schema::comments;
use crate::database::schema::comments::dsl::comments as table;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub content: String,
    pub author_id: u32,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub nb_votes: u32,
    pub parent_id: Option<u32>,
}

pub struct CommentMinima {
    pub post_id: u32,
    pub content: String,
    pub author_id: u32,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub nb_votes: u32,
    pub parent_id: u32,
}

impl Comment {
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }
}
