use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{Entity, PostEntity, TagEntity};
use crate::database::schema::{posts_tags, tags};
use crate::database::tables::{posts_tags_table as table, tags_table};
use crate::lib::consequence::*;

#[derive(Queryable, Serialize, Insertable, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelPostTagEntity {
    pub post_id: u32,
    pub tag_id: u32,
}

impl Entity for RelPostTagEntity {
    type Minima = Self;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        Err(EntityError::NotIdentifiable)?
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        // The database will give us an error if a post_id or tag_id do not
        // respect the foreign key validation
        match Self::select(conn, &minima)? {
            Some(entity) => Ok(Either::Left(entity)),
            None => {
                diesel::insert_into(table).values(minima).execute(conn)?;
                Ok(Either::Right(Self::select(conn, &minima)??))
            }
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                posts_tags::post_id
                    .eq(minima.post_id)
                    .and(posts_tags::tag_id.eq(minima.tag_id)),
            )
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, _conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, _conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }
}

impl RelPostTagEntity {
    pub fn tags_by_post_id(conn: &MysqlConnection, post_id: &u32) -> Consequence<Vec<TagEntity>> {
        table
            .inner_join(tags_table)
            .select((tags::id, tags::label))
            .filter(posts_tags::post_id.eq(post_id))
            .load::<TagEntity>(conn)
            .map(Ok)?
    }
}
