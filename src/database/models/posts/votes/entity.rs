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

// TODO : implement minima

impl RelPostVoteEntity {
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
