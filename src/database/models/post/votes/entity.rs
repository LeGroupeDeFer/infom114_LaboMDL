use crate::database::models::prelude::*;

use crate::database::schema::votes_posts;
use crate::database::tables::votes_posts_table as table;

use crate::lib::Consequence;
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

impl RelPostVoteEntity {
    pub fn get(conn: &MysqlConnection, post_id: &u32, user_id: &u32) -> Consequence<Option<Self>> {
        Ok(Some(
            table
                .filter(
                    votes_posts::post_id
                        .eq(post_id)
                        .and(votes_posts::user_id.eq(user_id)),
                )
                .first(conn)?,
        ))
    }

    /// Insert a vote for a given couple post and user
    /// `Either::Left` : The user had already voted for this post
    /// `Either::Right` : The user successfully added a new vote for this post
    pub fn insert_minima(
        conn: &MysqlConnection,
        minima: RelPostVoteMinima,
    ) -> Consequence<Either<Self, Self>> {
        Ok(match Self::get(conn, &minima.post_id, &minima.user_id)? {
            Some(rel_post_vote) => Either::Left(rel_post_vote),
            None => {
                diesel::insert_into(table).values(&minima).execute(conn)?;
                Either::Right(Self::get(conn, &minima.post_id, &minima.user_id)??)
            }
        })
    }

    pub fn sum_by_post_id(conn: &MysqlConnection, post_id: u32) -> Consequence<i64> {
        table
            .select(sum(votes_posts::vote_value))
            .filter(votes_posts::post_id.eq(post_id))
            .first::<Option<i64>>(conn)
            .unwrap_or(Some(0))
            .map(Ok)?
    }

    pub fn update(
        conn: &MysqlConnection,
        post_id: u32,
        user_id: u32,
        vote: i16,
    ) -> Consequence<()> {
        diesel::update(
            table.filter(
                votes_posts::post_id
                    .eq(post_id)
                    .and(votes_posts::user_id.eq(user_id)),
            ),
        )
        .set(votes_posts::vote_value.eq(vote))
        .execute(conn)?;
        Ok(())
    }

    pub fn delete(conn: &MysqlConnection, post_id: u32, user_id: u32) -> Consequence<()> {
        diesel::delete(
            table.filter(
                votes_posts::post_id
                    .eq(post_id)
                    .and(votes_posts::user_id.eq(user_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}
