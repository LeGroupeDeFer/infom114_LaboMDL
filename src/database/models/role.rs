use crate::database;
use crate::database::MyDbConn;
use crate::schema;

use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: u32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserRole {
    pub user: u32,
    pub role: u32,
}
