use chrono::NaiveDateTime;
use diesel::prelude::*;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::schema::votes_comments;
use crate::database::models::prelude::{CommentEntity, UserEntity};


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


//impl Identifiable for RelCommentVote {
//
//    type Id = (u32, u32);
//
//    fn id(self) -> Self::Id {
//        (self.comment_id, self.user_id)
//    }
//
//}


// TODO : implement minima
pub struct RelCommentVoteMinima {}


impl Entity for RelCommentVoteEntity {

    type Minima = RelCommentVoteMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> { unimplemented!() }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> { unimplemented!() }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }

}


