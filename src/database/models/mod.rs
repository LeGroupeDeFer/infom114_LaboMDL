//! # Models module
//!
//! A model object is a representation of a database row.
//! So a model struct is the "class" of that object and do respect the table
//! definition.
//!
//! Of course the struct can also implement some functionnalities that will
//! help the developper to do the basic CRUD operations.
//! In fact it's kinda of expected that the models behave like this and
//! implement some database operations.

pub mod address;
pub mod roles;
pub mod user;
