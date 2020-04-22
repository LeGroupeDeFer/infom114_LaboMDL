use crate::database::models::prelude::*;

// ------------------------------------------------------------------------------------- LOGIN FORM

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    pub access_token: String,
    pub refresh_token: String,
    pub user: PublicUser
}

// ------------------------------------------------------------------------------------ LOGOUT FORM

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutData {
    pub email: String,
    pub refresh_token: String,
}

// ---------------------------------------------------------------------------------------------- ?
#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
  pub email: String,
}

// ---------------------------------------------------------------------------------- REGISTER FORM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterData {
  pub email: String,
  pub password: String,
  pub firstname: String,
  pub lastname: String,
  pub address: Option<AddressMinima>,
  pub phone: Option<String>,
}

impl From<&RegisterData> for UserMinima {
    fn from(data: &RegisterData) -> UserMinima {
        UserMinima {
              email: data.email.clone(),
              password: data.password.clone(),
              firstname: data.firstname.clone(),
              lastname: data.lastname.clone(),
              address: None,
              phone: data.phone.clone(),
              activation_token: None,
              recovery_token: None,
              refresh_token: None
        }
    }
}

// -------------------------------------------------------------------------------- ACTIVATION FORM

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivationData {
    pub id: u32,
    pub token: String,
}

// ---------------------------------------------------------------------------------- RECOVERY FORM

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoveryData {
  pub id: u32,
  pub password: String,
  pub token: String
}

// --------------------------------------------------------------------------------- RESTORE FORM ?

#[derive(Serialize, Deserialize, Debug)]
pub struct RestoreData {
  pub email: String,
}

// ----------------------------------------------------------------------------------- REFRESH FORM

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshData {
    pub email: String,
    pub refresh_token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshSuccess {
    pub access_token: String,
    pub user: PublicUser,
    pub refresh_token: String
}
