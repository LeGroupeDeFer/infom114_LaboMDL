use crate::database::models::prelude::{PostEntity, UserEntity};
use crate::database::schema::votes_posts;

use crate::database::tables::votes_posts_table as table;

use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::MysqlConnection;

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
    pub fn get(conn: &MysqlConnection, post_id: u32, user_id: u32) -> Option<Self> {
        table
            .filter(
                votes_posts::post_id
                    .eq(post_id)
                    .and(votes_posts::user_id.eq(user_id)),
            )
            .first(conn)
            .ok()
    }

    pub fn insert_minima(conn: &MysqlConnection, minima: RelPostVoteMinima) -> bool {
        // fixme : maybe useless since we can catch exception with florian magical result
        if PostEntity::by_id(conn, minima.post_id).is_none()
            || UserEntity::by_id(conn, minima.user_id).is_none()
        {
            return false;
        }

        match Self::get(conn, minima.post_id, minima.user_id) {
            Some(rel_post_vote) => {
                if rel_post_vote.vote_value == minima.vote_value {
                    true
                } else {
                    diesel::update(
                        table.filter(
                            votes_posts::post_id
                                .eq(minima.post_id)
                                .and(votes_posts::user_id.eq(minima.user_id)),
                        ),
                    )
                    .set(votes_posts::vote_value.eq(minima.vote_value))
                    .execute(conn)
                    .is_ok()
                }
            }
            None => diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .is_ok(),
        }
    }

    pub fn sum_by_post_id(conn: &MysqlConnection, post_id: u32) -> i64 {
        table
            .select(sum(votes_posts::vote_value))
            .filter(votes_posts::post_id.eq(post_id))
            .first::<Option<i64>>(conn)
            .unwrap_or(Some(0))
            .unwrap()
    }

    pub fn update(conn: &MysqlConnection, post_id: u32, user_id: u32, vote: i16) -> bool {
        // TODO : use florian magical result to manage these cases
        diesel::update(
            table.filter(
                votes_posts::post_id
                    .eq(post_id)
                    .and(votes_posts::user_id.eq(user_id)),
            ),
        )
        .set(votes_posts::vote_value.eq(vote))
        .execute(conn)
        .is_ok()
    }

    pub fn delete(conn: &MysqlConnection, post_id: u32, user_id: u32) -> bool {
        diesel::delete(
            table.filter(
                votes_posts::post_id
                    .eq(post_id)
                    .and(votes_posts::user_id.eq(user_id)),
            ),
        )
        .execute(conn)
        .is_ok()
    }
}
