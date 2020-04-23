use super::entity::CapabilityEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Capability(CapabilityEntity);
