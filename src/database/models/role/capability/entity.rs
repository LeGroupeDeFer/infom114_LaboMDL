//! # RoleCapability module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `roles_capabilities` table

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::prelude::{CapabilityEntity, RoleEntity};
use crate::database::models::Entity;
use crate::database::schema::roles_capabilities;
use crate::database::schema::roles_capabilities::dsl::{self, roles_capabilities as table};
use crate::lib::Consequence;

/// The struct `RelRoleCapability` is the exact representation of the
/// `role_capabilities` table.
#[derive(
    Identifiable,
    Queryable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[table_name = "roles_capabilities"]
#[belongs_to(RoleEntity, foreign_key = "role_id")]
#[belongs_to(CapabilityEntity, foreign_key = "capability_id")]
pub struct RelRoleCapabilityEntity {
    pub id: u32,
    pub role_id: u32,
    pub capability_id: u32,
}

/// The struct `RelRoleCapabilityMinima` is used to insert some data inside the
/// `role_capabilities` table.
#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name = "roles_capabilities"]
pub struct RelRoleCapabilityMinima {
    pub role_id: u32,
    pub capability_id: u32,
}

impl Entity for RelRoleCapabilityEntity {
    type Minima = RelRoleCapabilityMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, minima)? {
            Some(past) => Left(past),
            None => {
                diesel::insert_into(table).values(minima).execute(conn)?;
                Right(Self::select(conn, minima)??)
            }
        })
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                dsl::role_id
                    .eq(&minima.role_id)
                    .and(dsl::capability_id.eq(&minima.capability_id)),
            )
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self)
            .set(self)
            .execute(conn)
            .map(|_| self)
            .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}
