use chrono::{Datelike, Utc};
use diesel::prelude::*;
use diesel::MysqlConnection;
use std::convert::TryFrom;

use crate::database::models::prelude::*;
use crate::database::schema::posts::dsl;
use crate::database::schema::posts_tags::dsl as posts_tags;
use crate::database::schema::tags::dsl as tags;
use crate::{database, lib};

use crate::database::models::post::WatchEventEntity;
use crate::database::tables::{
    posts_reports_table, posts_table as table, posts_tags_table, tags_table, watch_events_table,
};
use crate::database::SortOrder;
use crate::lib::{Consequence, EntityError};
use std::collections::{HashMap, HashSet};

// TODO - Move this in app state
const BASE: f64 = 1.414213562;
const EPOCH: u32 = 1577840400; // 01/01/2020
const EASING: u32 = 24 * 3600; // 1 day

/* Diesel helpers */

type AllColumns = (
    dsl::id,
    dsl::title,
    dsl::content,
    dsl::author_id,
    dsl::created_at,
    dsl::updated_at,
    dsl::deleted_at,
    dsl::hidden_at,
    dsl::locked_at,
    dsl::watched_at,
    dsl::votes,
    dsl::score,
    dsl::rank,
    dsl::kind,
);

const ALL_COLUMNS: AllColumns = (
    dsl::id,
    dsl::title,
    dsl::content,
    dsl::author_id,
    dsl::created_at,
    dsl::updated_at,
    dsl::deleted_at,
    dsl::hidden_at,
    dsl::locked_at,
    dsl::watched_at,
    dsl::votes,
    dsl::score,
    dsl::rank,
    dsl::kind,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub kind: String,
    pub created_at: String,
    pub updated_at: String,
    pub locked: bool,
    pub hidden: bool,
    pub deleted: bool,
    pub watched: bool,
    pub votes: u64,
    pub score: i64,
    pub rank: f64,
    pub flags: u64,
    pub author: User,
    pub tags: Vec<String>,
    pub comment_count: u64,
    pub comments: Vec<Comment>,
    pub user_vote: Option<i16>,
    pub user_flag: Option<bool>,
    pub watch_events: Vec<WatchEventEntity>,
    pub poll_info: Option<PostPoll>,
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
        tags: Vec<String>,
        keywords: Vec<String>,
        sort: Option<SortOrder>,
        kind: Option<String>,
        author: Option<u32>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Consequence<Vec<Self>> {
        let tags_length = tags.len() as u32;

        let mut query = table
            .inner_join(posts_tags_table.inner_join(tags_table))
            .left_join(watch_events_table)
            .select(ALL_COLUMNS)
            .filter(dsl::deleted_at.is_null())
            .into_boxed();

        /* ---------------------------------------------------------- HIDDEN */

        if !can_see_hidden {
            query = query.filter(dsl::hidden_at.is_null());
        }

        /* ----------------------------------------------- TAGS (INCOMPLETE) */

        if tags_length > 0 {
            query = query.filter(tags::label.eq_any(tags))
        }

        /* -------------------------------------------------------- KEYWORDS */

        for keyword in keywords {
            query = query.filter(
                dsl::title
                    .like(format!("%{}%", keyword))
                    .or(dsl::content.like(format!("%{}%", keyword))),
            );
        }

        /* ------------------------------------------------------------ KIND */

        let kid: Option<u8> = kind
            .and_then(|v| if &*v == "all" { None } else { Some(v) }) // Option<String>
            .map(|v| PostKind::try_from(v)) // Option<Result<PostKind>>
            .transpose()? // Option<PostKind>
            .map(|k| k.into());

        if let Some(id) = kid {
            query = query.filter(dsl::kind.eq(id));
        }

        /* ---------------------------------------------------------- AUTHOR */

        if let Some(author_id) = author {
            query = query.filter(dsl::author_id.eq(author_id));
        }

        /* -------------------------------------------------------- ORDER BY */

        query = match sort.unwrap_or(SortOrder::HighRank) {
            SortOrder::New => query.order(dsl::created_at.desc()),
            SortOrder::Old => query.order(dsl::created_at.asc()),
            SortOrder::HighScore => query.order((dsl::score.desc(), dsl::created_at.desc())),
            SortOrder::LowScore => query.order((dsl::score.asc(), dsl::created_at.desc())),
            SortOrder::HighRank => {
                query.order((dsl::rank.desc(), dsl::score.desc(), dsl::created_at.desc()))
            }
            SortOrder::LowRank => {
                query.order((dsl::rank.asc(), dsl::score.asc(), dsl::created_at.desc()))
            }
        };

        /* ---------------------------------------------------- END OF QUERY */

        let results = query.load::<PostEntity>(conn)?;

        Ok(Self::filter_out_tags(results, tags_length, limit, offset))
    }

    fn filter_out_tags(
        posts: Vec<Self>,
        number: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Vec<Self> {
        let mut tab: HashMap<u32, u32> = HashMap::new();
        let mut check_duplicate: HashSet<u32> = HashSet::new();

        for post in posts.iter() {
            let e = tab.entry(post.id).or_insert(0);
            *e += 1;
        }

        let filtered_posts = posts
            .into_iter()
            .filter(|entity| tab[&entity.id] >= number && check_duplicate.insert(entity.id))
            .collect::<Vec<Self>>();

        let from = offset.unwrap_or(0) as usize;
        let to = filtered_posts.len().min(
            limit
                .map(|l| from + l as usize)
                .unwrap_or(filtered_posts.len()),
        );

        filtered_posts[from..to].to_vec()
    }

    pub fn get_report_by_year(
        conn: &MysqlConnection,
        year: i32,
    ) -> Consequence<Vec<ActivityReport>> {
        struct EasyBoy {
            pub post: PostEntity,
            pub comments: Vec<CommentEntity>,
            pub votes: Vec<RelPostVoteEntity>,
        }

        let mut tab: HashMap<u32, ActivityReport> = HashMap::new();
        let months = lib::months();
        for i in 1..=12 {
            tab.insert(
                i,
                ActivityReport {
                    month: months[&i].to_string(),
                    new: 0,
                    interaction: 0,
                },
            );
        }

        for easy_boy in PostEntity::all(conn)?
            .into_iter()
            .map(move |post_entity| -> Consequence<EasyBoy> {
                let comments = CommentEntity::by_post_id(conn, &post_entity.id, true)?;
                let votes = RelPostVoteEntity::by_post_id(conn, &post_entity.id)?;
                Ok(EasyBoy {
                    post: post_entity,
                    comments,
                    votes,
                })
            })
            .filter(|object| object.is_ok())
            .map(move |object| object.unwrap())
            .filter(|easy_boy| {
                easy_boy.post.created_at.year() == year
                    || easy_boy.post.updated_at.year() == year
                    || (easy_boy.post.hidden_at.is_some()
                        && easy_boy.post.hidden_at.unwrap().year() == year)
                    || (easy_boy.post.locked_at.is_some()
                        && easy_boy.post.locked_at.unwrap().year() == year)
                    || easy_boy.comments.iter().any(|comment| {
                        comment.created_at.year() == year || comment.updated_at.year() == year
                    })
                    || easy_boy
                        .votes
                        .iter()
                        .any(|vote| vote.voted_at.year() == year)
            })
            .collect::<Vec<EasyBoy>>()
            .iter()
        {
            if easy_boy.post.created_at.year() == year {
                tab.get_mut(&easy_boy.post.created_at.month())?.new += 1;
            }

            if easy_boy.post.updated_at.year() == year {
                tab.get_mut(&easy_boy.post.updated_at.month())?.interaction += 1;
            }

            if easy_boy.post.hidden_at.is_some() && easy_boy.post.hidden_at.unwrap().year() == year
            {
                tab.get_mut(&easy_boy.post.hidden_at.unwrap().month())?
                    .interaction += 1;
            }

            if easy_boy.post.locked_at.is_some() && easy_boy.post.locked_at.unwrap().year() == year
            {
                tab.get_mut(&easy_boy.post.locked_at.unwrap().month())?
                    .interaction += 1;
            }

            for comment in easy_boy.comments.iter() {
                if comment.created_at.year() == year {
                    tab.get_mut(&comment.created_at.month())?.interaction += 1;
                }

                if comment.updated_at.year() == year {
                    tab.get_mut(&comment.updated_at.month())?.interaction += 1;
                }
            }

            for vote in easy_boy.votes.iter() {
                if vote.voted_at.year() == year {
                    tab.get_mut(&vote.voted_at.month())?.interaction += 1;
                }
            }
        }

        Ok(tab
            .into_iter()
            .map(move |(_, value)| value)
            .collect::<Vec<ActivityReport>>())
    }

    pub fn get_deleted(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(table.filter(dsl::deleted_at.is_not_null()).load(conn)?)
    }

    pub fn by_title(conn: &MysqlConnection, title: &str) -> Consequence<Vec<Self>> {
        Ok(table
            .filter(dsl::deleted_at.is_null().and(dsl::title.eq(title)))
            .load(conn)?)
    }

    pub fn by_tag(conn: &MysqlConnection, tag_id: &u32) -> Consequence<Vec<Self>> {
        Ok(table
            .inner_join(posts_tags_table)
            .filter(dsl::deleted_at.is_null().and(posts_tags::tag_id.eq(tag_id)))
            .load::<(Self, RelPostTagEntity)>(conn)?
            .into_iter()
            .map(move |(entity, _)| entity)
            .collect::<Vec<Self>>())
    }

    pub fn by_author_id(conn: &MysqlConnection, user_id: &u32) -> Consequence<Vec<Self>> {
        Ok(table
            .filter(dsl::deleted_at.is_null().and(dsl::author_id.eq(user_id)))
            .load(conn)?)
    }

    /// Delete a post permanently (not used)
    pub fn hard_delete(&self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }

    pub fn upvote(&mut self, conn: &MysqlConnection, user_id: &u32, vote: i32) -> Consequence<()> {
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
        RelPostVoteEntity::sum_by_post_id(&conn, &self.id)
    }

    pub fn count_votes(&self, conn: &MysqlConnection) -> Consequence<u64> {
        Ok(RelPostVoteEntity::count_by_post_id(&conn, &self.id)? as u64)
    }

    pub fn calculate_rank(&self) -> f64 {
        let elapsed = self.watched_at.unwrap_or(self.created_at).timestamp() as u32 - EPOCH;
        let logarithm = (self.votes as f64).log(BASE);
        let order = logarithm.max(1.0);
        order + (elapsed / EASING) as f64
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

    pub fn toggle_watch(&mut self, conn: &MysqlConnection) -> Consequence<()> {
        if self.watched_at.is_none() {
            self.watched_at = Some(Utc::now().naive_local());
        } else {
            self.watched_at = None;
        };

        self.rank = self.calculate_rank();
        self.update(conn)?;
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

    pub fn all_flagged(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        Ok(table
            .inner_join(posts_reports_table)
            .filter(dsl::deleted_at.is_null())
            .load::<(Self, RelPostReportEntity)>(conn)?
            .into_iter()
            .map(move |(entity, _)| entity)
            .collect::<Vec<Self>>())
    }

    pub fn get_flag_report(conn: &MysqlConnection) -> Consequence<Vec<ReportedPost>> {
        let mut tab: HashMap<u32, ReportedPost> = HashMap::new();

        for (post_entity, rel_report_post_entity) in table
            .inner_join(posts_reports_table)
            .filter(dsl::deleted_at.is_null())
            .load::<(Self, RelPostReportEntity)>(conn)?
            .iter()
        {
            let post_report = tab.entry(post_entity.id).or_insert(ReportedPost {
                post: Post::from(post_entity.clone()),
                count_flag: 0,
                reasons: vec![],
            });
            post_report.count_flag += 1;
            if let Some(value) = &rel_report_post_entity.reason {
                post_report.reasons.push(value.to_string());
            }
        }

        Ok(tab
            .into_iter()
            .map(move |(_, entity)| entity)
            .collect::<Vec<ReportedPost>>())
    }

    pub fn watch_now(&mut self) {
        self.watched_at = Some(Utc::now().naive_local());
        self.rank = self.calculate_rank();
    }
}

impl Post {
    pub fn all(
        conn: &MysqlConnection,
        can_see_hidden: bool,
        tags: Vec<String>,
        keywords: Vec<String>,
        sort: Option<SortOrder>,
        kind: Option<String>,
        author: Option<u32>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Consequence<Vec<Self>> {
        let entities = PostEntity::get_all(
            conn,
            can_see_hidden,
            tags,
            keywords,
            sort,
            kind,
            author,
            limit,
            offset,
        )?;
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

        if let Some(user) = UserEntity::by_id(conn, user_id).unwrap_or(None) {
            if user.has_capability(conn, "comment:view_hidden") {
                self.comments = CommentEntity::by_post_id(conn, &self.id, true)
                    .unwrap_or(vec![])
                    .into_iter()
                    .map(move |entity| Comment::from(entity))
                    .collect::<Vec<Comment>>();

                self.comment_count = CommentEntity::count_by_post_id(conn, &self.id, true);
            }
        }

        if let Some(post_poll) = self.poll_info.as_mut() {
            post_poll.set_user_info(conn, user_id).unwrap();
        }
    }
}

impl From<PostEntity> for Post {
    fn from(pe: PostEntity) -> Self {
        let conn = database::connection(&database::url());
        let kind = PostKind::try_from(pe.kind).unwrap();
        let poll_info = if kind == PostKind::Poll {
            Some(PostPoll::try_from(&pe).unwrap())
        } else {
            None
        };
        // fixme : optimize db request
        Self {
            id: pe.id,
            title: pe.title.to_string(),
            content: pe.content.to_string(),
            kind: kind.into(),
            created_at: pe.created_at.to_string(),
            updated_at: pe.updated_at.to_string(),
            locked: pe.locked_at.is_some(),
            hidden: pe.hidden_at.is_some(),
            deleted: pe.deleted_at.is_some(),
            watched: pe.watched_at.is_some(),
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
            comment_count: CommentEntity::count_by_post_id(&conn, &pe.id, false),
            comments: CommentEntity::by_post_id(&conn, &pe.id, false)
                .unwrap()
                .into_iter()
                .map(move |comment_entity| Comment::from(comment_entity))
                .collect::<Vec<Comment>>(),
            user_vote: None,
            user_flag: None,
            // TODO - Check the extend to which we need to create a WatchEvent wrapper
            watch_events: WatchEventEntity::by_post_id(&conn, &pe.id).unwrap(),
            poll_info,
        }
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
