use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};

use rocket_contrib::templates::Template;

use crate::http::errors::templates::ErrorTemplateContext;
use crate::http::helpers::RequestType;
use crate::http::responders::api::ApiResponse;

#[derive(Debug)]
pub struct CatcherResponse {
    request_type: RequestType,
    status: Status,
    message: String,
}

impl CatcherResponse {
    pub fn new(request_type: RequestType, status: Status, message: String) -> Self {
        Self {
            request_type,
            status,
            message,
        }
    }
}

impl<'r> Responder<'r> for CatcherResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self.request_type {
            RequestType::Json => ApiResponse::error(self.status, &self.message).respond_to(&req),
            _ => {
                let error_context = ErrorTemplateContext {
                    code: &self.status.code.to_string(),
                    message: &self.message,
                };
                // return error template with error
                Response::build_from(
                    Template::render("error", error_context)
                        .respond_to(&req)
                        .unwrap(),
                )
                .status(self.status)
                .ok()
            }
        }
    }
}
