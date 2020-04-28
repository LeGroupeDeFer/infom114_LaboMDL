//! # Models module
//!
//! A model object is a representation of a database row.
//! So a model struct is the "class" of that object and do respect the table
//! definition.
//!
//! Every model module is structured like so :
//!     - `entity.rs` contains the ORM needed struct and this struct MUST
//!         implement the `Entity` trait.
//!     - `name.rs` contains a struct that is meant to be used across the
//!         application
//!     - `form.rs` (optional) contains all data structs needed
//!     - a child model module (optional)    
//!
//! The prelude module groups all structs that are not "forms"

pub mod address;
pub mod capability;
pub mod comment;
pub mod post;
pub mod prelude;
pub mod role;
pub mod tag;
pub mod token;
pub mod user;

use diesel::MysqlConnection;
use either::*;

use crate::lib::consequence::*;

pub trait Entity: Sized {
    type Minima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>>;

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>>;

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>>;

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>>;

    // Synonym to add but errors if the record was already present
    fn insert_new(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Self> {
        let addition = Self::insert(conn, minima)?;
        if let Right(record) = addition {
            Ok(record)
        } else {
            Err(Error::EntityError(EntityError::Duplicate))
        }
    }

    // Synonym to add but does not differentiate whether the input was present or not
    fn insert_either(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Self> {
        Self::insert(conn, minima).map(|insertion| insertion.into_inner())
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self>;

    fn delete(self, conn: &MysqlConnection) -> Consequence<()>;
}
