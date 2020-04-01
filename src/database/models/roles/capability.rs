use crate::database::schema::capabilities;
use crate::database::schema::capabilities::dsl::capabilities as table;
use crate::database::Data;
use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "capabilities"]
pub struct Capability {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "capabilities"]
pub struct CapabilityMinima {
    pub name: String,
}

impl Capability {
    /* ------------------------------- STATIC ------------------------------ */

    pub fn from_id(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    pub fn from_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        table
            .filter(capabilities::name.eq(&name))
            .first::<Self>(conn)
            .ok()
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    /// Get the Capability record that fits the `minima` given.
    pub fn select_minima(conn: &MysqlConnection, minima: &CapabilityMinima) -> Option<Self> {
        table
            .filter(capabilities::name.eq(&minima.name))
            .first::<Self>(conn)
            .ok()
    }

    pub fn insert_minima(conn: &MysqlConnection, minima: &CapabilityMinima) -> Data<Self> {
        if let Some(past) = Self::select_minima(conn, minima) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::select_minima(conn, minima)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }
}

pub const CAPABILITIES: [&str; 22] = [
    "user:delete",
    "user:update",
    "user:view",
    "role:add",
    "role:update",
    "role:delete",
    "role:view",
    "role:update_capability",
    "role:assign_to_user",
    "post:create",
    "post:delete",
    "post:update",
    "post:flag",
    "post:unflag",
    "post:comment",
    "post:upvote",
    "comment:create",
    "comment:update",
    "comment:delete",
    "comment:flag",
    "comment:unflag",
    "post:view_report",
];
