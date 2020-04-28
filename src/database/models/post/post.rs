use diesel::prelude::*;
use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::MysqlConnection;

use crate::database::schema::posts::dsl::{self, posts as table};
use crate::database::models::post::RelPostVoteEntity;
use super::entity::PostEntity;


//#[derive(Debug, Serialize, Deserialize)]
//pub struct Post {
//    pub id: u32,
//    pub title: String,
//    pub content: String,
//    pub created_on: String,
//    pub updated_on: String,
//    pub locked_on: String,
//    pub hidden: bool,
//    pub votes: u32,
//    pub score: i32,
//    pub author: User,
//    pub tags: Vec<String>,
//    pub comments: Vec<Comment>,
//    pub self_vote: u32,
//}


impl PostEntity {

    /// Get `author_id` from a `post_id`
    pub fn get_author_id_by_post_id(conn: &MysqlConnection, post_id: u32) -> Option<u32> {
        table
            .find(post_id)
            .select(dsl::author_id)
            .first(conn)
            .ok()
    }

    /// Delete a post permanently (not used)
    pub fn hard_delete(&self, conn: &MysqlConnection) {
        diesel::delete(self).execute(conn).unwrap();
    }

    pub fn upvote(&self, conn: &MysqlConnection, user_id: u32, vote: i32) -> Option<i64> {
        // update rel score
        match vote {
            i if i == -1 || i == 1 => {
                RelPostVoteEntity::update(&conn, self.id, user_id, i as i16);
            }
            0 => {
                RelPostVoteEntity::delete(&conn, self.id, user_id);
            }
            _ => panic!("TODO : improve this error management"), // TODO
        }

        // get post score
        let new_score = self.calculate_score(&conn);

        // update self
        diesel::update(self)
            .set(dsl::score.eq(new_score))
            .execute(conn)
            .unwrap();
        Some(new_score)
    }

    pub fn calculate_score(&self, conn: &MysqlConnection) -> i64 {
        RelPostVoteEntity::sum_by_post_id(&conn, self.id)
    }

    pub fn toggle_visibility(&self, conn: &MysqlConnection) {
        if self.hidden_at.is_some() {
            let nope: Option<NaiveDateTime> = None;
            diesel::update(self)
                .set(dsl::hidden_at.eq(nope))
                .execute(conn)
                .unwrap();
        } else {
            diesel::update(self)
                .set(dsl::hidden_at.eq(now))
                .execute(conn)
                .unwrap();
        }
    }

    pub fn toggle_lock(&self, conn: &MysqlConnection) {
        if self.locked_at.is_some() {
            let nope: Option<NaiveDateTime> = None;
            diesel::update(self)
                .set(dsl::locked_at.eq(nope))
                .execute(conn)
                .unwrap();
        } else {
            diesel::update(self)
                .set(dsl::locked_at.eq(now))
                .execute(conn)
                .unwrap();
        }
    }

}