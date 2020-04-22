use chrono::NaiveDateTime;

use crate::database::schema::votes_comments;
use crate::database::tables::votes_comments_table as table;

use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
pub struct RelCommentVoteEntity {
    comment_id: u32,
    user_id: u32,
    voted_at: NaiveDateTime,
    vote_value: bool,
}

impl RelCommentVoteEntity {
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }
}

// TODO : implement minima
