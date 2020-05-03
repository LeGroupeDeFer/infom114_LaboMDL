use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::io::Cursor;
use std::result::Result as StdResult;

use crate::lib::consequence::{AuthError, EntityError, Error, JWTErrorKind, TokenError, UserError};

fn response_code(error: &Error) -> u16 {
    match error {
        Error::NotFound => 404,
        Error::DatabaseError(_) => 500,
        Error::BCryptError(_) => 500,
        Error::EntityError(e) => match e {
            EntityError::Duplicate => 409,

            // in cases a `by_id` function is called on a not identifiable StructEntity
            // or similar cases
            EntityError::NotIdentifiable => 500,
            EntityError::EmptyAttribute => 422,
            EntityError::InvalidID => 400,
            EntityError::InvalidAttribute => 400,
            EntityError::UnknownKind => 400
        },
        Error::TokenError(e) => match e {
            TokenError::Collision => 500,
            TokenError::Consumed => 403,
            TokenError::Expired => 401,
            TokenError::InvalidHash => 401,
        },
        Error::UserError(e) => match e {
            UserError::InvalidEmail => 422,
        },
        Error::JWTError(e) => match e.kind() {
            JWTErrorKind::InvalidToken => 401,
            JWTErrorKind::ExpiredSignature => 401,
            JWTErrorKind::InvalidIssuer => 401,
            JWTErrorKind::InvalidSubject => 401,
            _ => 500,
        },
        Error::AuthError(e) => match e {
            AuthError::InvalidIDs => 401,
            AuthError::Inactive => 403,
            AuthError::InvalidToken => 401,
            AuthError::AlreadyActivated => 401, // AlreadyActivated is mapped to the same err code as InvalidToken
            AuthError::MissingHeader => 400,
            AuthError::InvalidHeader => 400,
            AuthError::MissingCapability => 403,
        },
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
