//! # User role module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `users_roles` table

use crate::database::schema::roles;
use crate::database::schema::users_roles;
use crate::database::tables::{roles_table, users_roles_table};
use crate::database::Data;

use crate::database::models::prelude::{Role, User};

use diesel::prelude::*;
use diesel::MysqlConnection;

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

impl RelUserRole {
    /// Helper to get the roles of a user
    pub fn get_roles_from_user(conn: &MysqlConnection, user: &User) -> Vec<Role> {
        let roles_id = RelUserRole::belonging_to(user).select(users_roles::role_id);
        roles_table
            .filter(roles::id.eq_any(roles_id))
            .load::<Role>(conn)
            .expect("problem fetching roles from user")
    }

    /// Constructor of `RelUserRole` based on a user id and a role id
    pub fn get(conn: &MysqlConnection, user_id: u32, role_id: u32) -> Option<Self> {
        users_roles_table
            .filter(
                users_roles::user_id
                    .eq(user_id)
                    .and(users_roles::role_id.eq(role_id)),
            )
            .first(conn)
            .ok()
    }

    /// Insert a new row inside the `users_roles` table.
    pub fn add_role_for_user(
        conn: &MysqlConnection,
        user: &User,
        role: &Role,
    ) -> Data<RelUserRole> {
        match Self::get(&conn, user.id, role.id) {
            Some(e) => Data::Existing(e),
            None => {
                diesel::insert_into(users_roles_table)
                    .values(&RelUserRoleMinima {
                        user_id: user.id,
                        role_id: role.id,
                    })
                    .execute(conn)
                    .expect("error while inserting rel user role");
                Data::Inserted(
                    Self::get(&conn, user.id, role.id)
                        .expect("Insert succesfful but error while fetching it"),
                )
            }
        }
    }

    pub fn delete(self, conn: &MysqlConnection) {
        diesel::delete(&self).execute(conn).unwrap();
    }
}
