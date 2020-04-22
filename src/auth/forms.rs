use crate::database::models::address::AddressMinima;
use crate::database::models::users::user::UserMinima;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<AddressMinima>,
    pub phone: Option<String>,
}

impl RegisterData {
    pub fn user(&self) -> UserMinima {
        UserMinima {
            email: self.email.clone(),
            password: self.password.clone(),
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            address: None,
            phone: self.phone.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivationData {
    pub token: String,
    pub id: u32,
}
