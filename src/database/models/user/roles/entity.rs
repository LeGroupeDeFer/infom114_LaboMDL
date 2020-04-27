//! # User role module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `users_roles` table
use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{Result, Role, User};
use crate::database::models::Entity;

use crate::database::schema::users_roles;


/// The struct `RelUserRole` is the exact representation of the
/// `users_roles` table.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "users_roles"]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
pub struct RelUserRole {
    pub id: u32,
    pub user_id: u32,
    pub role_id: u32,
}


/// The struct `RelUserRoleMinima` is used to insert some data inside the
/// `users_roles` table.
#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name = "users_roles"]
pub struct RelUserRoleMinima {
    pub user_id: u32,
    pub role_id: u32,
}


impl Entity for RelUserRole {

    type Minima = RelUserRoleMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        unimplemented!()
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        unimplemented!()
    }

}
