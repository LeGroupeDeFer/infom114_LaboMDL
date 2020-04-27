use diesel::prelude::*;
use diesel::MysqlConnection;

use super::entity::Tag;
use crate::database::schema::tags::dsl::{self, tags as table};


impl Tag {
    // by_label :: (MysqlConnection) -> Option<Tag>
    pub fn by_label(conn: &MysqlConnection, label: &str) -> Option<Self> {
        table.filter(dsl::label.eq(label)).first(conn).ok()
    }
}