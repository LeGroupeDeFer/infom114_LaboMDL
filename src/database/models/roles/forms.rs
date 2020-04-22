//! # Forms module
//!
//! Here are grouped every struct that is needed somewhere
//! The purpose is to add some semantic to a group of attributes, to give
//! some structure (LOL)
//!
//! Of course those structs are related to some kind of role management

/// The `RoleData` struct is mainly used while checking if the JSON that
/// the client send is correctly formatted and full.
#[derive(Serialize, Deserialize, Debug)]
pub struct RoleData {
    pub name: String,
    pub color: String,
    pub capabilities: Vec<CapabilityData>,
}
