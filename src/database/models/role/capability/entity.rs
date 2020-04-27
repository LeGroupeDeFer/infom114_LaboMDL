//! # RoleCapability module
//!
//! In this module we'll go through the models needed to fetch an insert data
//! inside the `roles_capabilities` table

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::Entity;
use crate::database::models::prelude::{Result, Capability, Role};
use crate::database::schema::roles_capabilities;
use crate::database::schema::roles_capabilities::dsl::{self, roles_capabilities as table};


/// The struct `RelRoleCapability` is the exact representation of the
/// `role_capabilities` table.
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "roles_capabilities"]
#[belongs_to(Role, foreign_key = "role_id")]
#[belongs_to(Capability, foreign_key = "capability_id")]
pub struct RelRoleCapability {
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

impl Entity for RelRoleCapability {

    type Minima = RelRoleCapabilityMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> { table.load(conn).map(Ok)? }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        table
            .filter(
                dsl::role_id.eq(&minima.role_id)
                    .and(dsl::capability_id.eq(&minima.capability_id))
            )
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}