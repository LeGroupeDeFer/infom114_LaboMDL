use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::lib::consequence::Consequence;
use super::entity::CapabilityEntity;

use crate::database::schema::capabilities::dsl::{self, capabilities as table};

impl CapabilityEntity {
    /// Constructor of `Capability` from a role name
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Consequence<Option<Self>> {
        table
            .filter(dsl::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }
}