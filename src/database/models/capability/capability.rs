use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::database::models::result::Result;
use super::entity::Capability;

use crate::database::schema::capabilities::dsl::{self, capabilities as table};

impl Capability {
    /// Constructor of `Capability` from a role name
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Result<Option<Self>> {
        table
            .filter(dsl::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }
}