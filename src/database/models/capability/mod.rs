pub mod capability;
pub mod entity;
pub mod forms;

pub use capability::*;
pub use entity::*;
pub use forms::*;

/// All the capability of the application
pub const CAPABILITIES: [&str; 11] = [
    "role:manage",
    "user:manage_role",
    "users:view",
    "tag:update",
    "tag:delete",
    "post:update",
    "post:delete",
    "post:lock",
    "post:hide",
    "post:view_hidden",
    "post:edit_locked",
];
