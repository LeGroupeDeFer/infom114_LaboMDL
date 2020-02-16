#[derive(FromForm)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}
