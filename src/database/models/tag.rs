use crate::database::Data;

use crate::database::schema::tags;
use crate::database::schema::tags::dsl::tags as table;
use diesel::MysqlConnection;
use diesel::prelude::*;

/* ---------------------------------- Tag ---------------------------------- */

#[derive(Queryable, Serialize, Debug, Insertable)]
pub struct Tag {
    pub label: String,
}

impl Tag {
    // all :: (MysqlConnection) -> Vec<User>
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    // by_label :: (MysqlConnection) -> Option<Tag>
    pub fn by_label(conn: &MysqlConnection, label: &str) -> Option<Tag> {

        println!("{:?}",label);

        if let Ok(tag) = table.filter(tags::label.eq(label)).first(conn) {
            Some(tag)
        } else {
            None
        }   
    }

    // available_label :: (MysqlConnection) -> Boolean
    pub fn available_label(conn: &MysqlConnection, label: &str) -> bool {
        Tag::by_label(conn, label).is_none()
    }

    pub fn insert_tag(conn: &MysqlConnection, tag: &Tag) -> Data<Self> {
        if let Some(past) = Self::by_label(conn, &tag.label) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(tag)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::by_label(conn, &tag.label)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }
}