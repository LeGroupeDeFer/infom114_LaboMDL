/// The `CapabilityData` struct is only used here above and could be
/// replaced with the stuct `roles::capability::CapabilityMinima`
/// its kept that way because here we are independent of what could
/// happend in the other mod and we do not need to import the namespace
#[derive(Serialize, Deserialize, Debug)]
pub struct CapabilityData {
    pub name: String,
}
