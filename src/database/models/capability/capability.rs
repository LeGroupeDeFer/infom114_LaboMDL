use diesel::prelude::*;
use diesel::MysqlConnection;

use super::entity::CapabilityEntity;
use crate::lib::consequence::Consequence;

use crate::database::schema::capabilities::dsl::{self, capabilities as table};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Capability {
    pub id: u32,
    pub name: String,
}

impl From<CapabilityEntity> for Capability {
    fn from(ce: CapabilityEntity) -> Self {
        Self {
            id: ce.id,
            name: ce.name,
        }
    }
}

impl CapabilityEntity {
    /// Constructor of `CapabilityEntity` from a role name
    pub fn by_name(conn: &MysqlConnection, name: &str) -> Consequence<Option<Self>> {
        table
            .filter(dsl::name.eq(name))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    pub fn with_names(conn: &MysqlConnection, names: &Vec<&str>) -> Consequence<Vec<Self>> {
        table
            .filter(dsl::name.eq_any(names))
            .load::<Self>(conn)
            .map(Ok)?
    }
}
