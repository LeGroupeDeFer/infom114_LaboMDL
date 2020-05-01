use rocket::http::Status;
use rocket::Request;

use crate::http::helpers::RequestType;
use crate::http::responders::catcher::CatcherResponse;

pub fn collect() -> Vec<rocket::Catcher> {
    vec!()
}

//#[catch(401)]
pub fn unauthorized(req: &Request) -> CatcherResponse {
    let request_type = RequestType::guess(&req);
    CatcherResponse::new(
        request_type,
        Status::Unauthorized,
        "Unauthorized".to_string(),
    )
}

//#[catch(403)]
pub fn forbidden(req: &Request) -> CatcherResponse {
    let request_type = RequestType::guess(&req);
    CatcherResponse::new(request_type, Status::Forbidden, "Forbidden".to_string())
}

//#[catch(404)]
pub fn not_found(req: &Request) -> CatcherResponse {
    let request_type = RequestType::guess(&req);
    CatcherResponse::new(request_type, Status::NotFound, "Not found".to_string())
}

//#[catch(500)]
pub fn internal_server_error(req: &Request) -> CatcherResponse {
    let request_type = RequestType::guess(&req);
    CatcherResponse::new(
        request_type,
        Status::InternalServerError,
        "Internal Server Error".to_string(),
    )
}
