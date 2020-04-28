use diesel::MysqlConnection;
use chrono::Utc;
use jsonwebtoken::{self as jwt, Validation};

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

use crate::conf::AppState;
use crate::database::models::prelude::*;
use crate::lib::consequence::*;


pub const TOKEN_PREFIX: &'static str = "Bearer ";

/* --------------------------- Exposed submodules -------------------------- */

pub mod forms;
pub use forms::*;

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

    pub fn create(conn: &MysqlConnection, user: &UserEntity, lifetime: &u32) -> Consequence<Self> {
        let now = Utc::now().timestamp();
        Ok(Auth {
            iss: "Unanimity".to_string(),
            iat: now,
            exp: now + (*lifetime as i64),
            sub: user.id,
            cap: user.get_capabilities(&conn)?,
        })
    }

    pub fn token(&self, secret: &[u8]) -> Consequence<String> {
        jwt::encode(&jwt::Header::default(), self, secret).map(Ok)?
    }

    // ---------------------------- LOGIN / LOGOUT ----------------------------

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
        access_lifetime: &u32,
        refresh_lifetime: &u32
    ) -> Consequence<(Auth, TokenEntity, UserEntity)> {

        // Get user info
        let mut user = UserEntity::by_email(conn, email)??;
        let verification = user.verify(password)?;

        // Check the info
        if !verification {
            return Err(AuthError::InvalidIDs)?;
        } else if !user.active {
            return Err(AuthError::Inactive)?;
        }

        // Get or create the refresh token
        let mut refresh_token = user.refresh_token(conn)??;
        refresh_token.renew(conn, Some(refresh_lifetime), Some(&-1))?;
        user.last_connection = Utc::now().naive_local();
        user.update(conn)?;

        // We're good
        Ok((Auth::create(conn, &user, access_lifetime)?, refresh_token, user))
    }

    pub fn refresh(
        conn: &MysqlConnection,
        email: &str,
        hash: &str,
        access_lifetime: &u32,
        refresh_lifetime: &u32
    ) -> Consequence<(Auth, TokenEntity, UserEntity)> {
        let user = UserEntity::by_email(conn, email)??;
        let mut token = user.refresh_token(conn)??;

        token.verify(hash)?;
        if token.ttl() < token.lifespan() / 2 {
            token.renew(conn, Some(refresh_lifetime), Some(&-1))?;
        }

        Ok((Auth::create(conn, &user, access_lifetime)?, token, user))
    }

    pub fn logout(conn: &MysqlConnection, email: &str, hash: &str) -> Consequence<()> {
        let user = UserEntity::by_email(conn, email)??;
        let mut token = user.refresh_token(conn)?.map_or_else(
            || Err(AuthError::InvalidToken),
            |v| Ok(v)
        )?;
        token.verify(hash)?;
        token.revoke(conn)?;

        Ok(())
    }

    /// Check if the authenticated user has the requested capability
    pub fn has_capability(&self, conn: &MysqlConnection, capability: &str) -> bool {
        if let Some(capa) = CapabilityEntity::by_name(&conn, &capability).unwrap() {
            self.cap.contains(&capa)
        } else {
            // TODO : panic or log an error since the given capability potentially do not exist
            false
        }
    }
}

/* ------------------------- Traits implementations ------------------------ */

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = Error;

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

fn token_decode(token: &str, secret: &[u8]) -> Consequence<Auth> {
    jwt::decode(token, secret, &Validation::default())
        .map(|data| data.claims)
        .map(Ok)?
}

fn token_header(header: &str) -> Consequence<&str> {
    if !header.starts_with(TOKEN_PREFIX) {
        Err(AuthError::InvalidHeader)?;
    }
    Ok(&header[TOKEN_PREFIX.len()..])
}

fn request_auth(request: &Request, secret: &[u8]) -> Consequence<Auth> {
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
