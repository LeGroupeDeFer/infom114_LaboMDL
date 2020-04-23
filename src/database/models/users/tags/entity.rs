use crate::database::models::prelude::{TagEntity, UserEntity};

use crate::database::tables::{tags_subscription_table, tags_table, users_table};

use diesel::prelude::*;

#[derive(Identifiable, Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "tags_subscription"]
#[belongs_to(UserEntity, foreign_key = "user_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelUserTagEntity {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: u32,
}
// TODO : implement minima
