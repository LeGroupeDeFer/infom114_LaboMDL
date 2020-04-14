pub mod address;
pub mod result;
pub mod token;
pub mod user;
pub mod prelude;

use diesel::MysqlConnection;
use either::*;
use result::*;

pub trait Entity: Sized {

    type Minima;

    fn of(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>>;

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>>;

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>>;

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>>;

    // Synonym to add but errors if the record was already present
    fn insert_new(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Self> {
        let addition = Self::insert(conn, minima)?;
        if let Right(record) = addition {
            Ok(record)
        } else {
            Err(Error::EntityError(EntityError::Duplicate))
        }
    }

    // Synonym to add but does not differentiate whether the input was present or not
    fn insert_either(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Self> {
        Self::insert(conn, minima).map(|insertion| insertion.into_inner())
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self>;

    fn delete(self, conn: &MysqlConnection) -> Result<()>;
}
