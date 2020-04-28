use crate::database::models::prelude::{TagEntity, UserEntity};
use crate::database::schema::tags_subscription;
use diesel::MysqlConnection;

#[derive(Identifiable, Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "tags_subscription"]
#[belongs_to(UserEntity, foreign_key = "user_id")]
#[belongs_to(TagEntity, foreign_key = "tag_id")]
pub struct RelUserTagEntity {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: u32,
}

impl RelUserTagEntity {
    pub fn all(_conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }
}
// TODO : implement minima
