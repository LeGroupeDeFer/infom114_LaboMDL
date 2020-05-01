use chrono::NaiveDateTime;
use diesel::prelude::*;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::models::prelude::{CommentEntity, UserEntity};
use crate::database::schema::votes_comments;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
#[belongs_to(CommentEntity, foreign_key = "comment_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelCommentVoteEntity {
    comment_id: u32,
    user_id: u32,
    voted_at: NaiveDateTime,
    vote_value: bool,
}

// TODO : implement minima
pub struct RelCommentVoteMinima {}

impl Entity for RelCommentVoteEntity {
    type Minima = RelCommentVoteMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, _conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, _conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }
}
