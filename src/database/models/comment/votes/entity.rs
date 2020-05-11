use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::dsl::*;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::models::prelude::{CommentEntity, UserEntity};
use crate::database::schema::votes_comments;
use crate::database::tables::votes_comments_table as table;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
#[belongs_to(CommentEntity, foreign_key = "comment_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelCommentVoteEntity {
    pub comment_id: u32,
    pub user_id: u32,
    pub voted_at: NaiveDateTime,
    pub vote_value: i16,
}
#[derive(Insertable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_comments"]
pub struct RelCommentVoteMinima {
    pub comment_id: u32,
    pub user_id: u32,
    pub vote_value: i16,
}

impl Entity for RelCommentVoteEntity {
    type Minima = RelCommentVoteMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, &minima)? {
            Some(rel_comment_vote) => Either::Left(rel_comment_vote),
            None => {
                diesel::insert_into(table)
                    .values(minima.clone())
                    .execute(conn)?;
                Either::Right(Self::select(conn, &minima)??)
            }
        })
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                votes_comments::comment_id
                    .eq(minima.comment_id)
                    .and(votes_comments::user_id.eq(minima.user_id)),
            )
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(
            table.filter(
                votes_comments::comment_id
                    .eq(self.comment_id)
                    .and(votes_comments::user_id.eq(self.user_id)),
            ),
        )
        .set(votes_comments::vote_value.eq(self.vote_value))
        .execute(conn)
        .map(|_| self)
        .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(
            table.filter(
                votes_comments::comment_id
                    .eq(self.comment_id)
                    .and(votes_comments::user_id.eq(self.user_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl RelCommentVoteEntity {
    pub fn sum_by_comment_id(conn: &MysqlConnection, comment_id: u32) -> Consequence<i64> {
        Ok(table
            .select(sum(votes_comments::vote_value))
            .filter(votes_comments::comment_id.eq(comment_id))
            .first::<Option<i64>>(conn)?
            .unwrap_or(0))
    }

    pub fn count_by_comment_id(conn: &MysqlConnection, comment_id: u32) -> Consequence<u64> {
        table
            .select(count(votes_comments::user_id))
            .filter(votes_comments::comment_id.eq(comment_id))
            .first::<i64>(conn)
            .map(|v: i64| v as u64)
            .map(Ok)?
    }
}
