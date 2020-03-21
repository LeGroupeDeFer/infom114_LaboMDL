use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};

use rocket_contrib::json::{JsonError, JsonValue};

use crate::database::models::quick_response;

#[derive(Debug, Clone)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl ApiResponse {
    // TODO - Transmit errors to guards
    pub fn new(status: Status, json: JsonValue) -> Self {
        Self { status, json }
    }

    pub fn bad_request() -> Self {
        Self::new(
            Status::BadRequest,
            json!(quick_response::Info::new(Some(
                "I do not understand the language you are trying to communicate with.".to_string(),
            ))),
        )
    }

    pub fn success(status: Status, message: &str) -> Self {
        Self::new(
            status,
            json!(quick_response::Info::new(Some(message.to_string()))),
        )
    }

    pub fn simple_success(status: Status) -> Self {
        Self::new(status, json!(quick_response::Info::new(None)))
    }

    pub fn error(status: Status, message: &str) -> Self {
        Self::new(
            status,
            json!(quick_response::Info::new(Some(message.to_string()))),
        )
    }

    pub fn json_error(error: JsonError) -> Self {
        match error {
            JsonError::Io(_) => Self::bad_request(),
            JsonError::Parse(_, e) => Self::error(Status::UnprocessableEntity, &e.to_string()),
        }
    }

    pub fn db_error(error: diesel::result::Error) -> Self {
        Self::new(
            Status::InternalServerError,
            json!(quick_response::Info::new(Some(format!(
                "DB error : {}",
                error
            )))),
        )
    }
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
