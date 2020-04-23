use crate::database::models::prelude::{PostEntity, UserEntity};
use crate::database::schema::votes_posts;
use crate::database::tables::votes_posts_table as table;

use chrono::NaiveDateTime;

use diesel::prelude::*;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "votes_posts"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelPostVoteEntity {
    pub post_id: u32,
    pub user_id: u32,
    pub voted_at: NaiveDateTime,
    pub vote_value: bool,
}

// TODO : implement minima
