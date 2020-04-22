use crate::database::schema::posts_tags;
use crate::database::tables::posts_tags_table as table;

use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table("posts_tags")]
pub struct RelPostTagEntity {
    post_id: u32,
    tag_id: u32,
}

// TODO : implement minima
