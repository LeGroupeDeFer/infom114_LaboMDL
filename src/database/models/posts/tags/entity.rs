use crate::database::models::prelude::{PostEntity, TagEntity};
use crate::database::schema::{posts_tags, tags};
use crate::database::tables::{posts_tags_table as table, tags_table};
use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelPostTagEntity {
    pub post_id: u32,
    pub tag_id: u32,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
pub struct RelPostTagMinima {
    pub post_id: u32,
    pub tag_id: u32,
}

impl RelPostTagEntity {
    pub fn all(_conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }

    pub fn get(conn: &MysqlConnection, post_id: u32, tag_id: u32) -> Option<Self> {
        table
            .filter(
                posts_tags::post_id
                    .eq(post_id)
                    .and(posts_tags::tag_id.eq(tag_id)),
            )
            .first(conn)
            .ok()
    }

    pub fn delete(conn: &MysqlConnection, post_id: u32, tag_id: u32) -> bool {
        diesel::delete(
            table.filter(
                posts_tags::post_id
                    .eq(post_id)
                    .and(posts_tags::tag_id.eq(tag_id)),
            ),
        )
        .execute(conn)
        .is_ok()
    }

    pub fn insert(conn: &MysqlConnection, post_id: u32, tag_id: u32) -> bool {
        // assert both entities exist
        if PostEntity::by_id(conn, post_id).is_none() || TagEntity::by_id(conn, tag_id).is_none() {
            return false;
        }

        // create minima
        let minima = RelPostTagMinima { post_id, tag_id };

        // insert only if new
        match Self::get(conn, post_id, tag_id) {
            Some(_) => true,
            None => {
                diesel::insert_into(table)
                    .values(minima)
                    .execute(conn)
                    .expect("Error inserting post tag");
                true
            }
        }
    }

    pub fn tags_by_post_id(conn: &MysqlConnection, post_id: u32) -> Vec<TagEntity> {
        table
            .inner_join(tags_table)
            .select((tags::id, tags::label))
            .filter(posts_tags::post_id.eq(post_id))
            .load::<TagEntity>(conn)
            .unwrap_or(vec![])
    }
}
