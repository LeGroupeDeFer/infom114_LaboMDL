use chrono::NaiveDateTime;
use diesel::prelude::*;
use either::*;

use crate::database::models::Entity;
use crate::database::models::result::*;

use crate::database::schema::votes_comments;
use crate::database::models::prelude::{Comment, User};


#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
#[belongs_to(Comment, foreign_key = "comment_id")]
#[belongs_to(User, foreign_key = "user_id")]
pub struct RelCommentVote {
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


impl Entity for RelCommentVote {

    type Minima = RelCommentVoteMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> { unimplemented!() }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> { unimplemented!() }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        unimplemented!()
    }

}


