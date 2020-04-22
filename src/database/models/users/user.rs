use crate::database::models::prelude::{Address, Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub address: Address,
    pub roles: Vec<Role>,
}
