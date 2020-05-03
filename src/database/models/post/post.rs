use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database;
use crate::database::models::post::{RelPostReportEntity, RelPostReportMinima, RelPostVoteMinima};
use crate::database::models::prelude::*;
use crate::database::schema::posts::dsl;
use crate::database::schema::tags::dsl as tags;
use crate::database::tables::{posts_table as table, posts_tags_table, tags_table};
use crate::database::SortOrder;
use crate::lib::{Consequence, EntityError};


// TODO - Move this in app state
const BASE: f64 = 1.414213562;
const EPOCH: u64 = 1577840400; // 01/01/2020
const EASING: u64 = 24 * 3600; // 1 day


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
    pub rank: f64,
    pub flags: u64,
    pub author: User,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub user_vote: Option<i16>,
    pub user_flag: Option<bool>,
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

    pub fn get_all(
        conn: &MysqlConnection,
        can_see_hidden: bool,
        tag: Option<String>,
        search: Option<String>,
        sort: Option<SortOrder>,
        _tape: Option<String>, // type
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Consequence<Vec<Self>> {
        let mut query = table
            .left_join(posts_tags_table.inner_join(tags_table))
            .into_boxed();

        // filter out deleted
        query = query.filter(dsl::deleted_at.is_null());

        // filter out hidden
        if !can_see_hidden {
            query = query.filter(dsl::hidden_at.is_null());
        }

        // filter on tag
        if let Some(t) = tag {
            query = query.filter(tags::label.eq(t.to_string()));
        }

        // filter on the search term given (in title)
        if let Some(s) = search {
            query = query.filter(dsl::title.like(format!("%{}%", s)));
        }

        // order by
        if let Some(s) = sort {
            query = match s {
                SortOrder::New => query.order(dsl::created_at.desc()),
                SortOrder::Old => query.order(dsl::created_at.asc()),
                SortOrder::HighScore => query.order(dsl::score.desc()),
                SortOrder::LowScore => query.order(dsl::score.asc()),
                SortOrder::HighRank => query.order(dsl::rank.desc()),
                SortOrder::LowRank => query.order(dsl::rank.asc()),
            }
        }

        // limit the results
        if let Some(l) = limit {
            query = query.limit(l as i64);
        }

        // offset the results
        if let Some(o) = offset {
            if limit.is_none() {
                query = query.limit(10_000);
            }
            query = query.offset(o as i64);
        }

        Ok(query
            .load::<(PostEntity, Option<(RelPostTagEntity, TagEntity)>)>(conn)?
            .into_iter()
            .map(move |(post_entity, _)| post_entity)
            .collect::<Vec<Self>>())
    }

    pub fn get_deleted(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(table.filter(dsl::deleted_at.is_not_null()).load(conn)?)
    }

    pub fn by_title(conn: &MysqlConnection, title: &str) -> Consequence<Vec<Self>> {
        Ok(table
            .filter(dsl::deleted_at.is_null().and(dsl::title.eq(title)))
            .load(conn)?)
    }

    /// Delete a post permanently (not used)
    pub fn hard_delete(&self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }

    pub fn upvote(
        &mut self,
        conn: &MysqlConnection,
        user_id: &u32,
        vote: i32,
    ) -> Consequence<()> {
        let minima = RelPostVoteMinima {
            post_id: self.id,
            user_id: user_id.clone(),
            vote_value: vote as i16,
        };
        // update rel score
        match vote {
            -1 | 1 => match RelPostVoteEntity::select(&conn, &minima)? {
                Some(mut vote_entity) => {
                    vote_entity.vote_value = minima.vote_value;
                    vote_entity.update(&conn)?;
                }
                None => {
                    RelPostVoteEntity::insert_new(&conn, &minima)?;
                }
            },
            0 => match RelPostVoteEntity::select(&conn, &minima)? {
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
        self.rank = self.calculate_rank();

        // update self
        self.update(&conn)?;
        Ok(())
    }

    pub fn calculate_score(&self, conn: &MysqlConnection) -> Consequence<i64> {
        RelPostVoteEntity::sum_by_post_id(&conn, self.id)
    }

    pub fn count_votes(&self, conn: &MysqlConnection) -> Consequence<u64> {
        Ok(RelPostVoteEntity::count_by_post_id(&conn, self.id)? as u64)
    }

    pub fn calculate_rank(&self) -> f64 {
        let elapsed = self.created_at.timestamp() as u64 - EPOCH;
        let logarithm = (self.votes as f64).log(BASE);
        let order = logarithm;

        order + (elapsed / EASING) as f64
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

    pub fn add_tag(&self, conn: &MysqlConnection, tag_id: &u32) -> Consequence<()> {
        let minima = RelPostTagEntity {
            post_id: self.id,
            tag_id: *tag_id,
        };
        RelPostTagEntity::insert_either(conn, &minima)?;
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

    pub fn report(
        &self,
        conn: &MysqlConnection,
        user_id: &u32,
        reason: Option<String>,
    ) -> Consequence<()> {
        let minima = RelPostReportMinima {
            post_id: self.id,
            user_id: user_id.clone(),
            reason,
        };
        RelPostReportEntity::insert_new(conn, &minima)?;
        Ok(())
    }

    pub fn remove_report(&self, conn: &MysqlConnection, user_id: &u32) -> Consequence<()> {
        let minima = RelPostReportMinima {
            post_id: self.id,
            user_id: user_id.clone(),
            reason: None,
        };
        match RelPostReportEntity::select(conn, &minima)? {
            Some(entity) => {
                entity.delete(conn)?;
            }
            None => Err(EntityError::InvalidID)?,
        }
        Ok(())
    }
}

impl Post {
    pub fn all(
        conn: &MysqlConnection,
        can_see_hidden: bool,
        tag: Option<String>,
        search: Option<String>,
        sort: Option<SortOrder>,
        tape: Option<String>, // type
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Consequence<Vec<Self>> {
        let entities =
            PostEntity::get_all(conn, can_see_hidden, tag, search, sort, tape, limit, offset)?;
        let posts = entities
            .into_iter()
            .map(move |post_entity| Post::from(post_entity))
            .collect::<Vec<Self>>();
        Ok(posts)
    }

    pub fn set_user_info(&mut self, conn: &MysqlConnection, user_id: &u32) {
        let vote_minima = RelPostVoteMinima {
            post_id: self.id,
            user_id: user_id.clone(),
            vote_value: 0,
        };

        let user_vote = RelPostVoteEntity::select(conn, &vote_minima)
            .unwrap_or(None)
            .map_or(0, |vote| vote.vote_value);

        self.user_vote = Some(user_vote);

        let report_minima = RelPostReportMinima {
            post_id: self.id,
            user_id: user_id.clone(),
            reason: None,
        };

        self.user_flag = Some(
            RelPostReportEntity::select(&conn, &report_minima)
                .unwrap_or(None)
                .is_some(),
        );
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
            rank: pe.rank,
            flags: RelPostReportEntity::count_by_post_id(&conn, &pe.id).unwrap(),
            author: UserEntity::by_id(&conn, &pe.author_id)
                .unwrap()
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            tags: RelPostTagEntity::tags_by_post_id(&conn, &pe.id)
                .unwrap()
                .iter()
                .map(|tag_entity| tag_entity.label.to_string())
                .collect::<Vec<String>>(),
            comments: CommentEntity::by_post_id(&conn, &pe.id)
                .unwrap()
                .drain(..)
                .map(|comment_entity| Comment::from(comment_entity))
                .collect::<Vec<Comment>>(),
            user_vote: None,
            user_flag: None,
        }
    }
}
