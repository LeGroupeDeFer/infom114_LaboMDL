use rocket::http::Status;
use rocket::Request;

use crate::http::helpers::RequestType;
use crate::http::responders::catcher::CatcherResponse;

#[catch(404)]
pub fn not_found(req: &Request) -> CatcherResponse {
    let request_type = RequestType::guess(&req);
    CatcherResponse::new(request_type, Status::NotFound, "Not found".to_string())
}
