use super::entity::AddressEntity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address(AddressEntity);
