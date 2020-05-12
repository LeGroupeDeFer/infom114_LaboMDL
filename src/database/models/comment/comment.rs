use crate::database;
use crate::database::models::prelude::*;
use crate::database::schema::comments::dsl;
use crate::database::tables::comments_table as table;
use crate::database::SortOrder;
use crate::lib::{Consequence, EntityError};

use chrono::Utc;
use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub parent_id: Option<u32>,
    pub content: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
    // pub deleted_at: String,
    pub hidden: bool,
    pub locked: bool,
    pub deleted: bool,
    pub votes: u64,
    pub score: i64,
    pub flags: u64,
    pub replies: Vec<Comment>,
    pub user_vote: Option<i16>,
    pub user_flag: Option<bool>,
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

        let report_minima = RelCommentReportMinima {
            comment_id: self.id,
            user_id: user_id.clone(),
            reason: None,
        };

        self.user_flag = Some(
            RelCommentReportEntity::select(&conn, &report_minima)
                .unwrap_or(None)
                .is_some(),
        );
        // TODO return replies
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
            parent_id: ce.parent_id,
            content: ce.content.to_string(),
            author: UserEntity::by_id(&conn, &ce.author_id)
                .unwrap()
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            created_at: ce.created_at.to_string(),
            updated_at: ce.updated_at.to_string(),
            // deleted_at: ce.deleted_at.to_string(),
            hidden: ce.is_hidden(),
            locked: ce.is_locked(),
            deleted: ce.is_deleted(),
            votes: ce.votes,
            score: ce.score,
            flags: RelCommentReportEntity::count_by_comment_id(&conn, &ce.id).unwrap(),
            replies,
            user_vote: None,
            user_flag: None,
        }
    }
}

impl CommentEntity {
    pub fn by_post_id(
        conn: &MysqlConnection,
        post_id: &u32,
        can_see_hidden: bool,
    ) -> Consequence<Vec<Self>> {
        let mut query = table.into_boxed();
        query = query.filter(dsl::deleted_at.is_null());
        query = query.filter(dsl::post_id.eq(post_id));

        if !can_see_hidden {
            query = query.filter(dsl::hidden_at.is_null());
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

    pub fn toggle_visibility(&mut self, conn: &MysqlConnection) -> Consequence<()> {
        self.hidden_at = if self.hidden_at.is_none() {
            Some(Utc::now().naive_local())
        } else {
            None
        };
        self.update(conn)?;
        Ok(())
    }

    pub fn toggle_lock(&mut self, conn: &MysqlConnection) -> Consequence<()> {
        self.locked_at = if self.locked_at.is_none() {
            Some(Utc::now().naive_local())
        } else {
            None
        };
        self.update(conn)?;
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

        // get comment score
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

    pub fn report(
        &self,
        conn: &MysqlConnection,
        user_id: &u32,
        reason: Option<String>,
    ) -> Consequence<()> {
        let minima = RelCommentReportMinima {
            comment_id: self.id,
            user_id: user_id.clone(),
            reason,
        };
        RelCommentReportEntity::insert_new(conn, &minima)?;
        Ok(())
    }

    pub fn remove_report(&self, conn: &MysqlConnection, user_id: &u32) -> Consequence<()> {
        let minima = RelCommentReportMinima {
            comment_id: self.id,
            user_id: user_id.clone(),
            reason: None,
        };
        match RelCommentReportEntity::select(conn, &minima)? {
            Some(entity) => {
                entity.delete(conn)?;
            }
            None => Err(EntityError::InvalidID)?,
        }
        Ok(())
    }

    pub fn get_all(
        conn: &MysqlConnection,
        post_id: &u32,
        can_see_hidden: bool,
        sort: Option<SortOrder>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Consequence<Vec<Self>> {
        let mut query = table
            .select((
                dsl::id,
                dsl::post_id,
                dsl::parent_id,
                dsl::content,
                dsl::author_id,
                dsl::created_at,
                dsl::updated_at,
                dsl::deleted_at,
                dsl::hidden_at,
                dsl::locked_at,
                dsl::votes,
                dsl::score,
            ))
            .into_boxed();

        query = query.filter(dsl::post_id.eq(post_id));

        // filter out replies: keep only entries whoses parent_id is null
        query = query.filter(dsl::parent_id.is_null());
        
        // tquery = tquery.filter(dsl::parent_id.is_null());
        // filter out deleted
        query = query.filter(dsl::deleted_at.is_null());

        // filter out hidden
        if !can_see_hidden {
            query = query.filter(dsl::hidden_at.is_null());
        }

        // order by
        let s = sort.unwrap_or(SortOrder::HighRank);
        query = match s {
            SortOrder::New => query.order(dsl::created_at.desc()),
            SortOrder::Old => query.order(dsl::created_at.asc()),
            SortOrder::HighScore => query.order((dsl::score.desc(), dsl::created_at.desc())),
            SortOrder::LowScore => query.order((dsl::score.asc(), dsl::created_at.desc())),
            _ => query.order(dsl::created_at.asc()),
            // SortOrder::HighRank => {
            //     query.order((dsl::rank.desc(), dsl::score.desc(), dsl::created_at.desc()))
            // },
            // SortOrder::LowRank => {
            //     query.order((dsl::rank.asc(), dsl::score.asc(), dsl::created_at.desc()))
            // }
        };

        let results = query.load::<CommentEntity>(conn)?;

        let from = offset.unwrap_or(0) as usize;
        let to = results.len().min(match limit {
            Some(l) => from + l as usize,
            None => results.len(),
        } as usize);

        Ok(results[from..to].to_vec())
    }

}
