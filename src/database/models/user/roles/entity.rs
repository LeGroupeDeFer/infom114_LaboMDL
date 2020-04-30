//! # User role module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `users_roles` table
use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{RoleEntity, UserEntity};
use crate::database::models::Entity;

use crate::database::schema::users_roles;
use crate::lib::Consequence;

/// The struct `RelUserRole` is the exact representation of the
/// `users_roles` table.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "users_roles"]
#[belongs_to(UserEntity, foreign_key = "user_id")]
#[belongs_to(RoleEntity, foreign_key = "role_id")]
pub struct RelUserRoleEntity {
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

impl Entity for RelUserRoleEntity {
    type Minima = RelUserRoleMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, _conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, _conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }
}
