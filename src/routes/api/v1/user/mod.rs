//! # User controller mod
//!
//! Here are grouped every routes and controller that are used in user
//! management.

pub mod role;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    role::collect()
}
