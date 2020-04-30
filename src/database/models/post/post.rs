use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database;
use crate::database::models::prelude::*;
use crate::database::schema::posts::dsl::{self, posts as table};
use crate::lib::{Consequence, EntityError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub locked: bool,
    pub hidden: bool,
    pub deleted: bool,
    pub votes: u64,
    pub score: i64,
    pub author: User,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub user_vote: Option<i16>,
}

impl PostEntity {
    /// Get `author_id` from a `post_id`
    pub fn get_author_id_by_post_id(
        conn: &MysqlConnection,
        post_id: u32,
    ) -> Consequence<Option<u32>> {
        table
            .find(post_id)
            .select(dsl::author_id)
            .first(conn)
            .optional()
            .map(Ok)?
    }

    pub fn admin_all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(table.filter(dsl::deleted_at.is_null()).load(conn)?)
    }

    pub fn get_deleted(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table
            .filter(dsl::deleted_at.is_not_null())
            .load(conn)
            .map(Ok)?
    }

    /// Delete a post permanently (not used)
    pub fn hard_delete(&self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }

    pub fn upvote(
        &self,
        conn: &MysqlConnection,
        user_id: u32,
        vote: i32,
    ) -> Consequence<Option<i64>> {
        // update rel score
        match vote {
            i if i == -1 || i == 1 => {
                RelPostVoteEntity::update(&conn, self.id, user_id, i as i16)?;
            }
            0 => {
                RelPostVoteEntity::delete(&conn, self.id, user_id)?;
            }
            _ => Err(EntityError::InvalidAttribute)?,
        }

        // get post score
        let new_score = self.calculate_score(&conn)?;

        // update self
        diesel::update(self)
            .set(dsl::score.eq(new_score))
            .execute(conn)
            .map(|_| Some(new_score))
            .map(Ok)?
    }

    pub fn calculate_score(&self, conn: &MysqlConnection) -> Consequence<i64> {
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

    pub fn add_tag(&self, conn: &MysqlConnection, tag_id: &u32) -> Consequence<()> {
        let minima = RelPostTagEntity {
            post_id: self.id,
            tag_id: *tag_id,
        };
        RelPostTagEntity::insert(conn, &minima)?;
        Ok(())
    }
}

impl Post {
    pub fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        let mut entities = PostEntity::all(conn)?;
        let posts = entities
            .drain(..)
            .map(|post_entity| Post::from(post_entity))
            .collect::<Vec<Self>>();
        Ok(posts)
    }

    pub fn admin_all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(PostEntity::admin_all(conn)?
            .drain(..)
            .map(|post_entity| Post::from(post_entity))
            .collect::<Vec<Self>>())
    }

    pub fn set_user_vote(&mut self, conn: &MysqlConnection, user_id: &u32) -> Consequence<()> {
        let user_vote =
            RelPostVoteEntity::get(conn, &self.id, &user_id)?.map_or(0, |vote| vote.vote_value);

        self.user_vote = Some(user_vote);
        Ok(())
    }
}

impl From<PostEntity> for Post {
    fn from(pe: PostEntity) -> Self {
        let conn = database::connection(&database::url());
        // fixme : optimize db request
        Self {
            id: pe.id,
            title: pe.title.to_string(),
            content: pe.content.to_string(),
            created_at: pe.created_at.to_string(),
            updated_at: pe.updated_at.to_string(),
            locked: pe.locked_at.is_some(),
            hidden: pe.hidden_at.is_some(),
            deleted: pe.deleted_at.is_some(),
            votes: pe.votes,
            score: pe.score,
            author: UserEntity::by_id(&conn, &pe.author_id)
                .unwrap()
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            tags: RelPostTagEntity::tags_by_post_id(&conn, pe.id)
                .iter()
                .map(|tag_entity| tag_entity.label.to_string())
                .collect::<Vec<String>>(),
            comments: CommentEntity::by_post_id(&conn, &pe.id)
                .unwrap()
                .drain(..)
                .map(|comment_entity| Comment::from(comment_entity))
                .collect::<Vec<Comment>>(),
            user_vote: None,
        }
    }
}
