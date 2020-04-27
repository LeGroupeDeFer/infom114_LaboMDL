//! # Roles module
//!
//! This module groups every models related to role management

pub mod capability;
pub mod entity;
pub mod forms;
pub mod role;

pub use entity::*;
pub use role::*;
pub use forms::*;
pub use capability::*;