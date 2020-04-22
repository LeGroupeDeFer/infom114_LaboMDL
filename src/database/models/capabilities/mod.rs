pub mod capability;
pub mod entity;
pub mod forms;

/// All the capabilities of the application
pub const CAPABILITIES: [&str; 4] = [
    "role:manage",
    "user:manage_role",
    "tag:update",
    "tag:delete",
];
