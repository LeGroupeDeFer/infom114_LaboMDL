use crate::database::Data;

use crate::database::models::result::*;
use crate::database::models::Entity;
use crate::database::schema::tags;
use crate::database::schema::tags::dsl::tags as table;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

/* ---------------------------------- Tag ---------------------------------- */
#[derive(Clone, Debug, Queryable, Identifiable, Serialize, Insertable)]
#[primary_key(id)]
#[table_name = "tags"]
pub struct TagEntity {
    pub id: u32,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "tags"]
pub struct TagMinima {
    pub label: String,
}

impl TagEntity {
    // all :: (MysqlConnection) -> Vec<User>
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    // by_label :: (MysqlConnection) -> Option<Tag>
    pub fn by_label(conn: &MysqlConnection, label: &str) -> Option<Self> {
        table.filter(tags::label.eq(label)).first(conn).ok()
    }

    pub fn insert(conn: &MysqlConnection, tag: &TagMinima) -> Data<Self> {
        if let Some(past) = Self::by_label(conn, &tag.label.to_lowercase()) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(tag.to_lowercase())
                .execute(conn)
                .expect("Failed tag insertion");
            Data::Inserted(
                Self::by_label(conn, &tag.label)
                    .expect("Tag insertion succeeded but could not be retrieved"),
            )
        }
    }

    pub fn update(&self, conn: &MysqlConnection, new_label: &str) -> Data<Self> {
        match Self::by_label(&conn, new_label) {
            Some(label) => Data::Existing(label),
            None => {
                diesel::update(self)
                    .set(tags::label.eq(new_label))
                    .execute(conn)
                    .unwrap();
                Data::Updated(
                    Self::by_label(&conn, &new_label)
                        .expect("Tag update succeeded but could not be retrieved"),
                )
            }
        }
    }

    pub fn delete(&self, conn: &MysqlConnection) {
        diesel::delete(self).execute(conn).unwrap();
    }
}

impl TagMinima {
    pub fn to_lowercase(&self) -> Self {
        Self {
            label: self.label.to_lowercase(),
        }
    }
}
