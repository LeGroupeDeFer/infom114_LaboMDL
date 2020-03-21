use crate::conf::AppState;
use crate::database::models::user::User;
use crate::database::Connection;
use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use jwt::Validation;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub iss: String, // Issuer (us)
    pub iat: i64,    // Issued at (timestamp)
    pub exp: i64,    // Expire (timestamp)
    pub sub: u32,    // Subject (id)
    pub cap: Vec<String>,
}

impl Auth {
    pub fn new(user: &User, length: i64) -> Self {
        let now = Utc::now().timestamp();
        Auth {
            iss: "Unanimity".to_string(),
            iat: now,
            exp: now + length,
            sub: user.id,
            cap: vec![], // TODO - User capabilites
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(&jwt::Header::default(), self, secret).expect("jwt encoding error")
    }

    // ---------------------------- LOGIN / LOGOUT ----------------------------

    pub fn login(conn: &Connection, email: &str, password: &str) -> Option<(Auth, User)> {
        let validity = Duration::weeks(2).num_seconds();
        if let Some(user) = User::by_email(conn, email) {
            if user.verify(password) {
                return Some((Auth::new(&user, validity), user));
            }
        }

        None
    }
}

const TOKEN_PREFIX: &'static str = "Bearer ";

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = String;

    // from_request :: Request -> Outcome<Claims, Error>
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let state: State<AppState> = request.guard().unwrap();
        match request_auth(request, &state.jwt_secret) {
            Ok(auth) => Outcome::Success(auth),
            Err(msg) => {
                println!("{}", msg);
                Outcome::Failure((Status::Forbidden, msg))
            }
        }
    }
}

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
