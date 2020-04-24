use crate::conf::AppState;
use crate::database::models::prelude::PostEntity;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request, State};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostGuard {
    entity: PostEntity,
}

impl<'a, 'r> FromRequest<'a, 'r> for PostGuard {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let _state: State<AppState> = request.guard().unwrap();
        let _post_id = request.route().unwrap();
        // TODO : implement

        unimplemented!()
    }
}
