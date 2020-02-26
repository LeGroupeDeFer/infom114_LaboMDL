use crate::schema::users;

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(FromForm, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct RegisterCredentials {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub street: Option<String>,
    pub number: Option<u32>,
    pub city: Option<String>,
    pub zipcode: Option<u32>,
    pub country: Option<String>,
    pub phone: Option<String>,
}
