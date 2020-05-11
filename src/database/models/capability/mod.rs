pub mod capability;
pub mod entity;
pub mod forms;

pub use capability::*;
pub use entity::*;
pub use forms::*;

/// All the capability of the application
pub const CAPABILITIES: [&str; 23] = [
    "role:manage",
    "user:manage_role",
    "users:view",
    "tag:create",
    "tag:update",
    "tag:delete",
    "post:update",
    "post:delete",
    "post:lock",
    "post:hide",
    "post:view_hidden",
    "post:edit_locked",
    "post:watch",
    "comment:update",
    "comment:delete",
    "comment:lock",
    "comment:hide",
    "comment:view_hidden",
    "comment:edit_locked",
    "comment:watch",
    "report:view",
    "report:view_flag",
    "admin:access",
];
