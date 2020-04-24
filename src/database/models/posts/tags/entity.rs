use crate::database::models::prelude::{PostEntity, TagEntity};
use crate::database::schema::posts_tags;

#[derive(Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelPostTagEntity {
    post_id: u32,
    tag_id: u32,
}

// TODO : implement minima
