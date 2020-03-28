#[derive(Serialize, Deserialize, Debug)]
pub struct RoleData {
    pub name: String,
    pub color: String,
    pub capabilities: Vec<CapabilityData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapabilityData {
    pub name: String,
}
