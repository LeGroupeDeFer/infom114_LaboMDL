use super::entity::CapabilityEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Capability {
    pub id: u32,
    pub label: String,
}

impl From<CapabilityEntity> for Capability {
    fn from(ce: CapabilityEntity) -> Self {
        Self {
            id: ce.id,
            label: ce.name,
        }
    }
}
