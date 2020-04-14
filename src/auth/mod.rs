use crate::conf::AppState;
use crate::database::models::prelude::*;
use diesel::prelude::*;
use diesel::MysqlConnection;
use chrono::Utc;
use jsonwebtoken as jwt;
use jwt::Validation;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

const TOKEN_PREFIX: &'static str = "Bearer ";

/* --------------------------- Exposed submodules -------------------------- */

pub mod forms;

/* -------------------------------- Structs -------------------------------- */

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub iss: String, // Issuer (us)
    pub iat: i64,    // Issued at (timestamp)
    pub exp: i64,    // Expire (timestamp)
    pub sub: u32,    // Subject (id)
    pub cap: Vec<String>,
}

/* ----------------------------- Implementation ---------------------------- */

impl Auth {

    pub fn new(user: &User, lifetime: &u32) -> Self {
        let now = Utc::now().timestamp();
        Auth {
            iss: "Unanimity".to_string(),
            iat: now,
            exp: now + (*lifetime as i64),
            sub: user.id,
            cap: vec![], // TODO - User capabilites
        }
    }

    pub fn token(&self, secret: &[u8]) -> Result<String> {
        jwt::encode(&jwt::Header::default(), self, secret).map(Ok)?
    }

    // ---------------------------- LOGIN / LOGOUT ----------------------------

    pub fn login(
        conn: &MysqlConnection,
        email: &str,
        password: &str,
        access_lifetime: &u32,
        refresh_lifetime: &u32
    ) -> Result<(Auth, Token, User)> {
        use crate::database::schema::users::dsl;

        // Get user info
        let mut user = User::by_email(conn, email)??;
        let verification = user.verify(password)?;

        // Check the info
        if !verification {
            return Err(AuthError::InvalidIDs)?;
        } else if !user.active {
            return Err(AuthError::Inactive)?;
        }

        // Get or create the refresh token
        let mut refresh_token = user.refresh_token(conn)??;
        refresh_token.renew(conn, Some(-1))?;
        user.last_connection = Utc::now().naive_local();
        user.update(conn)?;

        // We're good
        Ok((Auth::new(&user, access_lifetime), refresh_token, user))
    }

    pub fn refresh(
        conn: &MysqlConnection,
        email: &str,
        hash: &str,
        access_lifetime: &u32
    ) -> Result<Auth> {
        let user = User::by_email(conn, email)??;
        let token = user.refresh_token(conn)??;

        token.verify(hash)?;
        Ok(Auth::new(&user, access_lifetime))
    }

    pub fn logout(conn: &MysqlConnection, email: &str, hash: &str) -> Result<()> {
        let user = User::by_email(conn, email)??;
        let mut token = user.refresh_token(conn)?.ok_or(AuthError::InvalidToken)?;
        token.verify(hash).map_err(|_| AuthError::InvalidToken)?;
        token.revoke(conn)?;

        Ok(())
    }

}

/* ------------------------- Traits implementations ------------------------ */

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = Error;

    // from_request :: Request -> Outcome<Auth, Error>
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let state: State<AppState> = request.guard().unwrap();
        match request_auth(request, &state.jwt_secret) {
            Ok(auth) => Outcome::Success(auth),
            Err(msg) => Outcome::Failure((Status::Forbidden, msg)),
        }
    }
}

/* ------------------------------- Functions ------------------------------- */

fn token_decode(token: &str, secret: &[u8]) -> Result<Auth> {
    jwt::decode(token, secret, &Validation::default())
        .map(|data| data.claims)
        .map(Ok)?
}

fn token_header(header: &str) -> Result<&str> {
    if !header.starts_with(TOKEN_PREFIX) {
        Err(AuthError::InvalidHeader)?;
    }
    Ok(&header[TOKEN_PREFIX.len()..])
}

fn request_auth(request: &Request, secret: &[u8]) -> Result<Auth> {
    if let Some(header) = request.headers().get_one("authorization") {
        let token = token_header(header);
        token.and_then(|token| token_decode(token, secret))
    } else {
        Err(AuthError::MissingHeader)?
    }
}

pub fn secret(rocket: &rocket::Rocket) -> &str {
    rocket.config().get_str("jwt_secret").unwrap()
}
