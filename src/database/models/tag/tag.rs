use diesel::prelude::*;
use diesel::MysqlConnection;

use super::entity::TagEntity;
use crate::database::schema::tags::dsl::{self, tags as table};
use crate::lib::Consequence;

pub struct Tag {
    pub id: u32,
    pub label: String,
}

impl TagEntity {
    // by_label :: (MysqlConnection) -> Option<Tag>
    pub fn by_label(conn: &MysqlConnection, label: &str) -> Consequence<Option<Self>> {
        Ok(Some(table.filter(dsl::label.eq(label)).first(conn)?))
    }
}

impl From<TagEntity> for Tag {
    fn from(te: TagEntity) -> Self {
        Self {
            id: te.id,
            label: te.label.to_string(),
        }
    }
}
