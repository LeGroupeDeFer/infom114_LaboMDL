use crate::database::models::prelude::{Tag, User};
use crate::database::schema::tags_subscription;
use diesel::MysqlConnection;

#[derive(Identifiable, Queryable, Serialize, Associations, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "tags_subscription"]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Tag, foreign_key = "tag_id")]
pub struct RelUserTag {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: u32,
}

impl RelUserTag {
    pub fn all(_conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }
}
// TODO : implement minima
