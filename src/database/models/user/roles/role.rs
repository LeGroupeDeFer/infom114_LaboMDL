use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::schema::roles::dsl::{self as roles_dsl, roles as roles_table};
use crate::database::schema::users_roles::dsl::{self, users_roles as table};

use super::entity::{RelUserRoleEntity, RelUserRoleMinima};
use crate::database::models::prelude::{RoleEntity, UserEntity};
use crate::lib::Consequence;
use either::Either;

impl RelUserRoleEntity {
    /// Helper to get the roles of a user
    pub fn get_roles_by_user(
        conn: &MysqlConnection,
        user: &UserEntity,
    ) -> Consequence<Vec<RoleEntity>> {
        let roles_id = RelUserRoleEntity::belonging_to(user).select(dsl::role_id);
        roles_table
            .filter(roles_dsl::id.eq_any(roles_id))
            .load::<RoleEntity>(conn) // Vec<Role>
            .map(Ok)?
    }

    /// Constructor of `RelUserRole` based on a user id and a role id
    pub fn get(conn: &MysqlConnection, user_id: u32, role_id: u32) -> Consequence<Option<Self>> {
        table
            .filter(dsl::user_id.eq(user_id).and(dsl::role_id.eq(role_id)))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    /// Insert a new row inside the `users_roles` table.
    /// `Either::Left` means that the entity already existed in database
    /// `Either::Right` means that a new entity has been created
    pub fn add_role_for_user(
        conn: &MysqlConnection,
        user: &UserEntity,
        role: &RoleEntity,
    ) -> Consequence<Either<Self, Self>> {
        Ok(match Self::get(&conn, user.id, role.id)? {
            Some(e) => Either::Left(e),
            None => {
                diesel::insert_into(table)
                    .values(&RelUserRoleMinima {
                        user_id: user.id,
                        role_id: role.id,
                    })
                    .execute(conn)?;
                Either::Right(Self::get(&conn, user.id, role.id)??)
            }
        })
    }

    pub fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        let _size = diesel::delete(&self).execute(conn)?;
        Ok(())
    }
}
