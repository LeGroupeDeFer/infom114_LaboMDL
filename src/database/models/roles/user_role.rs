//! # User role module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `users_roles` table

use crate::database::models::result::Result;
use crate::database::schema::roles;
use crate::database::schema::roles::dsl::roles as table_roles;
use crate::database::schema::users_roles;
use crate::database::schema::users_roles::dsl::users_roles as table_users_roles;
use crate::database::Data;

use crate::database::models::roles::role::Role;
use crate::database::models::user::User;

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
    pub fn get_roles_from_user(conn: &MysqlConnection, user: &User) -> Result<Vec<Role>> {
        let roles_id = RelUserRole::belonging_to(user).select(users_roles::role_id);
        table_roles
            .filter(roles::id.eq_any(roles_id))
            .load::<Role>(conn)
            .map(Ok)?
    }

    /// Constructor of `RelUserRole` based on a user id and a role id
    pub fn get(conn: &MysqlConnection, user_id: u32, role_id: u32) -> Result<Option<Self>> {
        table_users_roles
            .filter(
                users_roles::user_id
                    .eq(user_id)
                    .and(users_roles::role_id.eq(role_id)),
            )
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    /// Insert a new row inside the `users_roles` table.
    pub fn add_role_for_user(
        conn: &MysqlConnection,
        user: &User,
        role: &Role,
    ) -> Result<Data<RelUserRole>> {
        Ok(match Self::get(&conn, user.id, role.id)? {
            Some(e) => Data::Existing(e),
            None => {
                diesel::insert_into(table_users_roles)
                    .values(&RelUserRoleMinima {
                        user_id: user.id,
                        role_id: role.id,
                    })
                    .execute(conn)?;
                Data::Inserted(Self::get(&conn, user.id, role.id)??)
            }
        })
    }

    pub fn delete(self, conn: &MysqlConnection) {
        diesel::delete(&self).execute(conn).unwrap();
    }
}
