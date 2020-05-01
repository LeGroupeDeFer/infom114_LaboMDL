use super::entity::AddressEntity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub id: u32,
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub country: String,
}

impl From<AddressEntity> for Address {
    fn from(ae: AddressEntity) -> Self {
        Self {
            id: ae.id,
            street: ae.street,
            number: ae.number,
            box_number: ae.box_number,
            city: ae.city,
            zipcode: ae.zipcode,
            country: ae.country,
        }
    }
}
