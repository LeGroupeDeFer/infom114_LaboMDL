use std::ops::Deref;

use crate::database::schema::posts;
use crate::database::Data;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;
use diesel::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub authorid: u32,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub nb_votes: u32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub authorid: u32,
}

impl Post {
    // get_all_posts :: (DBConnection) -> QueryResult<Vec<User>>
    pub fn get_all_posts(conn: &MysqlConnection) -> QueryResult<Vec<Post>> {
        posts::table.load::<Post>(conn.deref())
    }

    /// Get a post object by `post_id`
    pub fn get_post_by_id(conn: &MysqlConnection, post_id: u32) -> Option<Post> {
        if let Ok(a_post) = posts::table.filter(posts::id.eq(post_id)).first(conn) {
            Some(a_post)
        } else {
            None
        }
    }

    /// Get `author_id` from a `post_id`
    pub fn get_author_id_by_post_id(conn: &MysqlConnection, post_id: u32) -> Option<u32> {
        let author_id = posts::table
            .filter(posts::id.eq(post_id))
            .select(posts::authorid)
            .first(conn);

        if let Ok(id) = author_id {
            Some(id)
        } else {
            None
        }
    }

    /// Delete a post permanently (not used)
    pub fn permanent_delete_post(conn: &MysqlConnection, post_id: u32) -> Option<usize> {
        let post_id = diesel::delete(posts::table.filter(posts::id.eq(post_id))).execute(conn);
        if let Ok(id) = post_id {
            Some(id)
        } else {
            None
        }
    }

    /// Soft-delete a post, aka. change `deleted_at` column
    pub fn soft_delete_post(conn: &MysqlConnection, post_id: u32) {}

    /// Update a post
    pub fn update_post(
        conn: &MysqlConnection,
        post_id: u32,
        new_title: String,
        new_content: String,
    ) -> Option<usize> {
        let target = posts::table.filter(posts::id.eq(post_id));
        let update_res = diesel::update(target)
            .set((posts::title.eq(new_title), posts::content.eq(new_content)))
            .execute(conn);
        if let Ok(_) = update_res {
            Some(1)
        } else {
            None
        }
    }

    pub fn upvote(conn: &MysqlConnection, post_id: u32, change_vote: u32) -> Option<u32> {
        let target = posts::table.filter(posts::id.eq(post_id));
        let nb_votes: Result<u32, _> = target.select(posts::nb_votes).first(conn);

        if let Ok(old_nb_votes) = nb_votes {
            let new_nb_votes = old_nb_votes + change_vote;
            diesel::update(target)
                .set(posts::nb_votes.eq(new_nb_votes))
                .execute(conn)
                .unwrap();
            Some(new_nb_votes)
        } else {
            None
        }
    }
}
