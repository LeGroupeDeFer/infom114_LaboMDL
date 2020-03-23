//! # UNanimity binary
//!
//! This is the application entry point.
//!
//! An effort has been made to keep the code minimal here.

use unanimitylibrary;

/// Launch the rocket application
fn main() {
    unanimitylibrary::rocket(unanimitylibrary::ignite()).launch();
}
