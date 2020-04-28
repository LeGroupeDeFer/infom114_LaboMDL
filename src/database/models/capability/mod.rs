pub mod capability;
pub mod entity;
pub mod forms;

pub use capability::*;
pub use entity::*;
pub use forms::*;

/// All the capability of the application
pub const CAPABILITIES: [&str; 7] = [
    "role:manage",
    "user:manage_role",
    "users:view",
    "tag:update",
    "tag:delete",
    "post:update",
    "post:delete",
];
