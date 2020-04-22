use crate::database::schema::votes_posts;
use crate::database::tables::votes_posts_table as table;

use chrono::NaiveDateTime;

use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_posts"]
pub struct RelPostVoteEntity {
    pub post_id: u32,
    pub vote_author_id: u32,
    pub voted_at: NaiveDateTime,
    pub vote_value: bool,
}

// TODO : implement minima
