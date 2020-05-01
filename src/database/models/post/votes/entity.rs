use crate::database::models::prelude::*;

use crate::database::schema::votes_posts;
use crate::database::tables::votes_posts_table as table;

use crate::lib::{Consequence, EntityError};
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_posts"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelPostVoteEntity {
    pub post_id: u32,
    pub user_id: u32,
    pub voted_at: NaiveDateTime,
    pub vote_value: i16,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_posts"]
pub struct RelPostVoteMinima {
    pub post_id: u32,
    pub user_id: u32,
    pub vote_value: i16,
}

impl Entity for RelPostVoteEntity {
    type Minima = RelPostVoteMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        Err(EntityError::NotIdentifiable)?
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    /// Insert a vote for a given couple post and user
    /// `Either::Left` : The user had already voted for this post
    /// `Either::Right` : The user successfully added a new vote for this post
    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, &minima)? {
            Some(rel_post_vote) => Either::Left(rel_post_vote),
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
                votes_posts::post_id
                    .eq(minima.post_id)
                    .and(votes_posts::user_id.eq(minima.user_id)),
            )
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(
            table.filter(
                votes_posts::post_id
                    .eq(self.post_id)
                    .and(votes_posts::user_id.eq(self.user_id)),
            ),
        )
        .set(votes_posts::vote_value.eq(self.vote_value))
        .execute(conn)
        .map(|_| self)
        .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(
            table.filter(
                votes_posts::post_id
                    .eq(self.post_id)
                    .and(votes_posts::user_id.eq(self.user_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl RelPostVoteEntity {
    pub fn sum_by_post_id(conn: &MysqlConnection, post_id: u32) -> Consequence<i64> {
        table
            .select(sum(votes_posts::vote_value))
            .filter(votes_posts::post_id.eq(post_id))
            .first::<Option<i64>>(conn)
            .unwrap_or(Some(0))
            .map(Ok)?
    }

    pub fn count_by_post_id(conn: &MysqlConnection, post_id: u32) -> Consequence<u64> {
        table
            .select(count(votes_posts::user_id))
            .filter(votes_posts::post_id.eq(post_id))
            .first::<i64>(conn)
            .map(|v: i64| v as u64)
            .map(Ok)?
    }
}
