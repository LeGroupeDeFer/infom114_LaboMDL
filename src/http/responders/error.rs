use std::result::Result as StdResult;
use rocket::response::{Response, Responder};
use rocket::request::Request;
use rocket::http::{Status, ContentType};
use std::io::Cursor;

use crate::database::models::result::{
    StdError, Error, DieselError, EntityError, UserError, TokenError, JWTError, JWTErrorKind,
    AuthError
};

fn response_code(error: &Error) -> u16 {
    match error {
        Error::NotFound => 404,
        Error::DatabaseError(_) => 500,
        Error::BCryptError(_) => 500,
        Error::EntityError(e) => match e {
            EntityError::Duplicate => 409,
        },
        Error::TokenError(e) => match e {
            TokenError::Collision => 500,
            TokenError::Consumed => 403,
            TokenError::Expired => 403,
            TokenError::InvalidHash => 401
        },
        Error::UserError(e) => match e {
            UserError::InvalidEmail => 422
        },
        Error::JWTError(e) => match e.kind() {
            JWTErrorKind::InvalidToken => 401,
            JWTErrorKind::ExpiredSignature => 401,
            JWTErrorKind::InvalidIssuer => 401,
            JWTErrorKind::InvalidSubject => 401,
            _ => 500
        },
        Error::AuthError(e) => match e {
            AuthError::InvalidIDs => 401,
            AuthError::Inactive => 403,
            AuthError::AlreadyActivated => 401,
            AuthError::InvalidToken => 401,
            AuthError::MissingHeader => 400,
            AuthError::InvalidHeader => 400
        }
    }
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> StdResult<Response<'a>, Status> {
        let code = response_code(&self);
        let body = json!({ "reason": format!("{}", &self) }).to_string();
        let response = Response::build()
            .status(Status::raw(code))
            .header(ContentType::JSON)
            .sized_body(Cursor::new(body))
            .finalize();

        Ok(response)
    }
}
