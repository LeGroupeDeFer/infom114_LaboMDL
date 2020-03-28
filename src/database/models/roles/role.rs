use crate::database::Data;

use crate::database::models::roles::capability::Capability;

use crate::database::schema::roles;
use crate::database::schema::roles::dsl::roles as table;

use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "roles"]
pub struct Role {
    pub id: u32,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "roles"]
pub struct RoleMinima {
    pub name: String,
    pub color: String,
}

impl Role {
    /* ------------------------------- STATIC ------------------------------ */

    pub fn from_id(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    /// Get the Capability record that fits the `minima` given.
    pub fn from_name(conn: &MysqlConnection, name: &str) -> Option<Self> {
        table.filter(roles::name.eq(name)).first::<Self>(conn).ok()
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    pub fn insert_minima(conn: &MysqlConnection, minima: &RoleMinima) -> Data<Self> {
        if let Some(past) = Self::from_name(conn, &minima.name) {
            Data::Existing(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .expect("Failed address insertion");
            Data::Inserted(
                Self::from_name(conn, &minima.name)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }

    pub fn capabilities(&self, conn: &MysqlConnection) -> Vec<Capability> {
        unimplemented!()
    }

}
