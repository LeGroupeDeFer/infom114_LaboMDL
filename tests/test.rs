//! # Tests
//!
//! The tests are organized by folder.
//!
//! To run the tests, please limit the number of threads with
//! `cargo test -- --test-threads=1`.
//! Since the database is reset at the begining of each test, this is the best
//! way to prevent the use of shared state.
//!
//! Cargo by default only tests the files that are at the root of the `/tests`
//! directory.
//! So we have to add all the modules that are used for testing purposes.

mod auth;
mod comments;
mod init;
mod posts;
mod reports;
mod roles;
mod tags;
