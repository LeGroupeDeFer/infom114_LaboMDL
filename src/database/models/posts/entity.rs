use std::ops::Deref;

use crate::database::schema::posts;
use crate::database::Data;

use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use diesel::MysqlConnection;
use diesel::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct PostEntity {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub votes: u32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

impl PostEntity {
    /// Get all posts
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        posts::table.load::<Self>(conn.deref()).unwrap_or(vec![])
    }

    /// Get a post by its id
    pub fn by_id(conn: &MysqlConnection, post_id: u32) -> Option<Self> {
        posts::table.filter(posts::id.eq(post_id)).first(conn).ok()
    }

    /// Get `author_id` from a `post_id`
    pub fn get_author_id_by_post_id(conn: &MysqlConnection, post_id: u32) -> Option<u32> {
        posts::table
            .find(post_id)
            .select(posts::author_id)
            .first(conn)
            .ok()
    }

    pub fn select_minima(conn: &MysqlConnection, minima: &PostMinima) -> Option<Self> {
        posts::table
            .filter(
                posts::title
                    .eq(minima.title.clone())
                    .and(posts::content.eq(minima.content.clone())),
            )
            .first(conn)
            .ok()
    }

    pub fn insert_minima(conn: &MysqlConnection, minima: &PostMinima) -> Data<Self> {
        if let Some(past) = Self::select_minima(conn, minima) {
            Data::Existing(past)
        } else {
            diesel::insert_into(posts::table)
                .values(minima)
                .execute(conn)
                .expect("Error inserting Post");
            Data::Inserted(
                Self::select_minima(conn, &minima)
                    .expect("Post insertion succeeded but could not be retrieved"),
            )
        }
    }

    /// Delete a post permanently (not used)
    pub fn hard_delete(&self, conn: &MysqlConnection) {
        diesel::delete(self).execute(conn);
    }

    /// Soft-delete a post, aka. change `deleted_at` column
    pub fn delete(&self, conn: &MysqlConnection) {
        diesel::update(self)
            .set(posts::deleted_at.eq(now))
            .execute(conn)
            .unwrap();
    }

    /// Update a post
    pub fn update(&self, conn: &MysqlConnection, minima: &PostMinima) -> Option<usize> {
        diesel::update(self)
            .set((
                posts::title.eq(minima.title.clone()),
                posts::content.eq(minima.content.clone()),
            ))
            .execute(conn)
            .ok()
    }

    pub fn upvote(&self, conn: &MysqlConnection, change_vote: i32) -> Option<i32> {
        let new_score = self.score + change_vote;
        diesel::update(self)
            .set(posts::score.eq(new_score))
            .execute(conn)
            .unwrap();
        Some(new_score)
    }

    pub fn toggle_visibility(&self, conn: &MysqlConnection) {
        if self.hidden_at.is_some() {
            let nope: Option<NaiveDateTime> = None;
            diesel::update(self)
                .set(posts::hidden_at.eq(nope))
                .execute(conn)
                .unwrap();
        } else {
            diesel::update(self)
                .set(posts::hidden_at.eq(now))
                .execute(conn)
                .unwrap();
        }
    }

    pub fn toggle_lock(&self, conn: &MysqlConnection) {
        if self.locked_at.is_some() {
            let nope: Option<NaiveDateTime> = None;
            diesel::update(self)
                .set(posts::locked_at.eq(nope))
                .execute(conn)
                .unwrap();
        } else {
            diesel::update(self)
                .set(posts::locked_at.eq(now))
                .execute(conn)
                .unwrap();
        }
    }
}
