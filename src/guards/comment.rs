use crate::database;
use crate::database::models::prelude::*;
use crate::lib::{EntityError, Error};
use diesel::MysqlConnection;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentGuard {
    entity: CommentEntity,
}

impl CommentGuard {
    pub fn comment(&self) -> &CommentEntity {
        &self.entity
    }

    pub fn comment_clone(self) -> CommentEntity {
        self.entity
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CommentGuard {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn: MysqlConnection = database::connection(&database::url());
        let mut comment_id: Option<u32> = None;
        for (defined_route_fragment, real_route_fragment) in request
            .route()
            .unwrap()
            .uri
            .segments()
            .zip(request.uri().segments())
        {
            if defined_route_fragment == "<comment_id>" || defined_route_fragment == "<_comment_id>" {
                comment_id = Some(real_route_fragment.parse::<u32>().unwrap());
                break;
            }
        }
        match comment_id {
            Some(id) => match CommentEntity::by_id(&conn, &id).unwrap() {
                Some(comment) => Outcome::Success(Self { entity: comment }),
                None => Outcome::Failure((
                    Status::BadRequest,
                    Error::EntityError(EntityError::InvalidID),
                )),
            },
            None => Outcome::Forward(()),
        }
    }
}
