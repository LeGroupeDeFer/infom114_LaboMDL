use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{Entity, Post, Tag};
use crate::database::schema::posts_tags;
use crate::database::models::result::*;


#[derive(Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_tags"]
#[belongs_to(Post, foreign_key = "post_id")]
#[belongs_to(Tag, foreign_key = "tag_id")]
pub struct RelPostTag {
    post_id: u32,
    tag_id: u32,
}


impl Entity for RelPostTag {
    type Minima = ();

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        unimplemented!()
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        unimplemented!()
    }
}