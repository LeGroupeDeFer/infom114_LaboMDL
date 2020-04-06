use crate::database::Data;

use crate::database::schema::tags;
use crate::database::schema::tags::dsl::tags as table;
use diesel::MysqlConnection;
use diesel::prelude::*;

/* ---------------------------------- Tag ---------------------------------- */
#[derive(Debug, Queryable, Identifiable, Serialize, Insertable)]
#[primary_key(id)]
#[table_name = "tags"]
pub struct Tag {
    pub id: u32,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "tags"]
pub struct TagMinima {
    pub label: String,
}

impl Tag {
    // all :: (MysqlConnection) -> Vec<User>
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    // by_label :: (MysqlConnection) -> Option<Tag>
    pub fn by_label(conn: &MysqlConnection, label: &str) -> Option<Tag> {
        if let Ok(tag) = table.filter(tags::label.eq(label)).first(conn) {
            Some(tag)
        } else {
            None
        }   
    }

    pub fn insert(conn: &MysqlConnection, tag: &TagMinima) -> Data<Self> {
        if let Some(past) = Self::by_label(conn, &tag.label) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(tag)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::by_label(conn, &tag.label)
                    .expect("Address insertion succeeded but could not be retrieved"),
            )
        }
    }

    pub fn update(conn: &MysqlConnection, old_label: String, new_label:String) -> Data<Self> {
        if let Some(label_availability) = Self::by_label(conn, &new_label) {
            Data::Existing(label_availability)
        } else if let Some(past) = Self::by_label(conn, &old_label) {
            diesel::update(table.filter(tags::columns::label.eq(old_label))) // -> reference to the tag label to update TODO
            .set(tags::label.eq(new_label))
            .execute(conn)
            .unwrap();
            
            Data::Updated(past) 
        } else {
            Data::None            
        }
    }

    pub fn delete(conn: &MysqlConnection, label: String) -> Data<Self> {
        if let Some(tag) = Self::by_label(conn, &label) {
            //Delete it
            diesel::delete(table.filter(tags::columns::label.eq(label)))
            .execute(conn)
            .unwrap();
            
            Data::Deleted(tag)
        } else {
            //422
            Data::None
        }
    }

}
