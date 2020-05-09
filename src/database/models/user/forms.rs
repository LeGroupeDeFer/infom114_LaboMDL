use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: u32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,

    pub address: Option<u32>,
    pub phone: Option<String>,

    pub creation_date: NaiveDateTime,
    pub last_connection: NaiveDateTime,

    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountUserForm {
    pub total: u64,
    pub active: u64,
    pub connected: u64,
}
