use chrono::NaiveDateTime;

use crate::database::models::prelude::{CommentEntity, UserEntity};

use crate::database::schema::votes_comments;

use diesel::prelude::*;

#[derive(Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
#[belongs_to(CommentEntity, foreign_key = "comment_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelCommentVoteEntity {
    comment_id: u32,
    user_id: u32,
    voted_at: NaiveDateTime,
    vote_value: bool,
}

impl RelCommentVoteEntity {
    pub fn all(_conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }
}

// TODO : implement minima
