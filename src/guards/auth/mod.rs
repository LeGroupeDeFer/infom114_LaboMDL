use crate::conf::AppState;
use crate::database::models::prelude::{CapabilityEntity, UserEntity};

use chrono::{Duration, Utc};
use diesel::MysqlConnection;
use jsonwebtoken as jwt;
use jwt::Validation;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

pub const TOKEN_PREFIX: &'static str = "Bearer ";

/* --------------------------- Exposed submodules -------------------------- */

pub mod forms;

/* -------------------------------- Structs -------------------------------- */

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub iss: String, // Issuer (us)
    pub iat: i64,    // Issued at (timestamp)
    pub exp: i64,    // Expire (timestamp)
    pub sub: u32,    // Subject (id)
    pub cap: Vec<CapabilityEntity>,
}

/* ----------------------------- Implementation ---------------------------- */

impl Auth {
    pub fn new(conn: &MysqlConnection, user: &UserEntity, length: i64) -> Self {
        let now = Utc::now().timestamp();
        Auth {
            iss: "Unanimity".to_string(),
            iat: now,
            exp: now + length,
            sub: user.id,
            cap: user.get_capabilities(&conn),
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(&jwt::Header::default(), self, secret).expect("jwt encoding error")
    }

    /// Perform the login operation :
    /// check if the given email exists and is linked to a validated account
    /// and that the given password is correct for that user
    ///
    /// If so, the authentication process is completed and an `Auth` object is returned
    /// along with the `User` object
    pub fn login(
        conn: &MysqlConnection,
        email: &str,
        password: &str,
    ) -> Option<(Auth, UserEntity)> {
        let validity = Duration::weeks(2).num_seconds();
        if let Some(user) = UserEntity::by_email(conn, email) {
            if user.verify(password) {
                return Some((Auth::new(&conn, &user, validity), user));
            }
        }
        None
    }

    /// Check if the authenticated user has the requested capability
    pub fn has_capability(&self, conn: &MysqlConnection, capability: &str) -> bool {
        if let Some(capa) = CapabilityEntity::by_name(&conn, &capability) {
            self.cap.contains(&capa)
        } else {
            // TODO : panic or log an error since the given capability potentially do not exist
            false
        }
    }
}

/* ------------------------- Traits implementations ------------------------ */

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = String;

    // from_request :: Request -> Outcome<Auth, Error>
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let state: State<AppState> = request.guard().unwrap();
        match request_auth(request, &state.jwt_secret) {
            Ok(auth) => Outcome::Success(auth),
            Err(msg) => Outcome::Failure((Status::Forbidden, msg)),
        }
    }
}

/* ------------------------------- Functions ------------------------------- */

// token_decode :: (String, [Int]) -> Result<Claims, Error>
fn token_decode(token: &str, secret: &[u8]) -> Result<Auth, String> {
    jwt::decode(token, secret, &Validation::default())
        .map_err(|err| format!("Unable to decode token : {:?}", err))
        .map(|data| data.claims)
}

// token_header :: String -> Result<String, Error>
fn token_header(header: &str) -> Result<&str, String> {
    if header.starts_with(TOKEN_PREFIX) {
        Ok(&header[TOKEN_PREFIX.len()..])
    } else {
        Err(format!("Malformed authentication header: {:?}", header))
    }
}

// request_auth:: (Request, [Int]) -> Result<Claims, Error>
fn request_auth(request: &Request, secret: &[u8]) -> Result<Auth, String> {
    if let Some(header) = request.headers().get_one("authorization") {
        let token = token_header(header);
        token.and_then(|token| token_decode(token, secret))
    } else {
        Err("Missing authorization header".to_string())
    }
}

pub fn secret(rocket: &rocket::Rocket) -> &str {
    rocket.config().get_str("jwt_secret").unwrap()
}
