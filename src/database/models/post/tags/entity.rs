use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{Entity, PostEntity, TagEntity};
use crate::database::schema::posts_tags;
use crate::lib::consequence::*;


#[derive(Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelPostTagEntity {
    post_id: u32,
    tag_id: u32,
}


impl Entity for RelPostTagEntity {
    type Minima = ();

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }
}