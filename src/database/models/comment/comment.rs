use crate::database;
use crate::database::models::prelude::*;
use crate::database::schema::comments::dsl;
use crate::database::tables::comments_table as table;
use crate::lib::{Consequence, EntityError};

use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub content: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
    pub votes: u64,
    pub score: i64,
    pub replies: Vec<Comment>,
    pub user_vote: Option<i16>,
}

impl Comment {
    pub fn set_user_info(&mut self, conn: &MysqlConnection, user_id: &u32) {
        if let Some(user) = UserEntity::by_id(conn, user_id).unwrap_or(None) {
            self.replies = CommentEntity::by_comment_id(
                conn,
                &self.id,
                user.has_capability(conn, "comment:view_hidden"),
            )
            .unwrap_or(vec![])
            .into_iter()
            .map(move |entity| {
                let mut c = Comment::from(entity);
                c.set_user_info(conn, user_id);
                c
            })
            .collect::<Vec<Self>>();
        }
        let vote_minima = RelCommentVoteMinima {
            comment_id: self.id,
            user_id: user_id.clone(),
            vote_value: 0,
        };

        let user_vote = RelCommentVoteEntity::select(conn, &vote_minima)
            .unwrap_or(None)
            .map_or(0, |vote| vote.vote_value);

        self.user_vote = Some(user_vote);
    }
}

impl From<CommentEntity> for Comment {
    fn from(ce: CommentEntity) -> Self {
        let conn = database::connection(&database::url());
        let replies = CommentEntity::by_comment_id(&conn, &ce.id, false)
            .unwrap_or(vec![])
            .into_iter()
            .map(move |entity| Self::from(entity))
            .collect::<Vec<Self>>();

        // fixme : optimize db request
        Self {
            id: ce.id,
            post_id: ce.post_id,
            content: ce.content.to_string(),
            author: UserEntity::by_id(&conn, &ce.author_id)
                .unwrap()
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            created_at: ce.created_at.to_string(),
            updated_at: ce.updated_at.to_string(),
            votes: ce.votes,
            score: ce.score,
            replies,
            user_vote: None,
        }
    }
}

impl CommentEntity {
    pub fn by_post_id(
        conn: &MysqlConnection,
        post_id: &u32,
        hidden: bool,
    ) -> Consequence<Vec<Self>> {
        let mut query = table.into_boxed();
        query = query.filter(dsl::deleted_at.is_not_null());
        query = query.filter(dsl::post_id.eq(post_id));

        if !hidden {
            query = query.filter(dsl::hidden_at.is_not_null());
        }

        Ok(query.load(conn)?)
    }

    pub fn by_comment_id(
        conn: &MysqlConnection,
        comment_id: &u32,
        hidden: bool,
    ) -> Consequence<Vec<Self>> {
        let mut query = table.into_boxed();
        query = query.filter(dsl::deleted_at.is_not_null());
        query = query.filter(dsl::parent_id.eq(comment_id));

        if !hidden {
            query = query.filter(dsl::hidden_at.is_not_null());
        }

        Ok(query.load(conn)?)
    }

    pub fn toggle_visibility(&self, conn: &MysqlConnection) -> Consequence<()> {
        if self.hidden_at.is_some() {
            diesel::update(self)
                .set(dsl::hidden_at.eq(None as Option<NaiveDateTime>))
                .execute(conn)?;
        } else {
            diesel::update(self)
                .set(dsl::hidden_at.eq(now))
                .execute(conn)?;
        }

        Ok(())
    }
    pub fn toggle_lock(&self, conn: &MysqlConnection) -> Consequence<()> {
        if self.locked_at.is_some() {
            diesel::update(self)
                .set(dsl::locked_at.eq(None as Option<NaiveDateTime>))
                .execute(conn)?;
        } else {
            diesel::update(self)
                .set(dsl::locked_at.eq(now))
                .execute(conn)?;
        }
        Ok(())
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_locked(&self) -> bool {
        self.locked_at.is_some()
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden_at.is_some()
    }

    pub fn upvote(&mut self, conn: &MysqlConnection, user_id: &u32, vote: i32) -> Consequence<()> {
        let minima = RelCommentVoteMinima {
            comment_id: self.id,
            user_id: user_id.clone(),
            vote_value: vote as i16,
        };
        // update rel score
        match vote {
            -1 | 1 => match RelCommentVoteEntity::select(&conn, &minima)? {
                Some(mut vote_entity) => {
                    vote_entity.vote_value = minima.vote_value;
                    vote_entity.update(&conn)?;
                }
                None => {
                    RelCommentVoteEntity::insert_new(&conn, &minima)?;
                }
            },
            0 => match RelCommentVoteEntity::select(&conn, &minima)? {
                Some(vote_entity) => {
                    vote_entity.delete(&conn)?;
                }
                None => {}
            },
            _ => Err(EntityError::InvalidAttribute)?,
        }

        // get post score
        self.score = self.calculate_score(&conn)?;
        self.votes = self.count_votes(&conn)?;

        // update self
        self.update(&conn)?;
        Ok(())
    }

    pub fn calculate_score(&self, conn: &MysqlConnection) -> Consequence<i64> {
        RelCommentVoteEntity::sum_by_comment_id(&conn, self.id)
    }

    pub fn count_votes(&self, conn: &MysqlConnection) -> Consequence<u64> {
        Ok(RelCommentVoteEntity::count_by_comment_id(&conn, self.id)?)
    }
}
