//! # Role module
//!
//! Here will be grouped every structs that allows the representation of the
//! table `roles` and what is needed by rust to insert data in it.

use crate::database::models::Entity;
use crate::database::models::result::Result;

use crate::database::models::roles::capability::Capability;
use crate::database::models::roles::role_capability::RelRoleCapability;

use crate::database::schema::roles;
use crate::database::schema::roles::dsl::roles as table;

use crate::database::schema::roles_capabilities::{
    self, dsl::roles_capabilities as table_roles_capabilities,
};

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;


/// The struct `Role` is the perfect representation of the data that can be hold
/// in the `roles` table.
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "roles"]
pub struct Role {
    pub id: u32,
    pub name: String,
    pub color: String,
}

/// The struct `Roleminima` is needed by rust to perform an insert in the database
/// because the role id is auto incremented, we do not know it before inserting data.
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "roles"]
pub struct RoleMinima {
    pub name: String,
    pub color: String,
}

impl Entity for Role {

    type Minima = RoleMinima;

    /// Constructor based on the role id.
    fn by_id(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    /// Fetch and return all the roles present in database as a `Role` vector
    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    /// Insert data stored in the `RoleMinima` struct given in parameter inside the
    /// database1
    fn insert(conn: &MysqlConnection, minima: &RoleMinima) -> Result<Either<Self, Self>> {
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
            .filter(roles::name.eq(minima.name.clone()))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::roles::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }
}

impl Role {
    /* ------------------------------- STATIC ------------------------------ */

    /// Get the Capability record that fits the role name given.
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        table
            .filter(roles::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    /* ------------------------------- NOT SO STATIC ------------------------------ */

    /// Get the capabilities linked to a role
    pub fn capabilities(&self, conn: &MysqlConnection) -> Result<Vec<Capability>> {
        Ok(RelRoleCapability::get_capabilities_for_role(&conn, &self)?)
    }

    /// Clear the database stored capabilities linked to this role
    pub fn clear_capabilities(&self, conn: &MysqlConnection) {
        diesel::delete(table_roles_capabilities.filter(roles_capabilities::role_id.eq(self.id)))
            .execute(conn)
            .unwrap();
    }

    /// Update the role database informations
    pub fn update(&self, conn: &MysqlConnection, minima: &RoleMinima) {
        diesel::update(self)
            .set((roles::name.eq(&minima.name), roles::color.eq(&minima.color)))
            .execute(conn)
            .unwrap();
    }

    /// Delete the role in database
    /// This will first remove all capabilities linked to this role
    pub fn delete(self, conn: &MysqlConnection) {
        self.clear_capabilities(conn);
        diesel::delete(&self).execute(conn).unwrap();
    }
}
