use crate::database;
use crate::database::models::prelude::*;
use crate::lib::{EntityError, Error};
use diesel::MysqlConnection;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserGuard {
    entity: UserEntity,
}

impl UserGuard {
    pub fn user(&self) -> &UserEntity {
        &self.entity
    }

    pub fn user_clone(self) -> UserEntity {
        self.entity
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn: MysqlConnection = database::connection(&database::url());

        // `request.route().unwrap().uri` is the route that we've defined with a macro
        // meaning that this one contains the string `<user_id>´

        // `request.uri()` is the route that the router got from the client
        // meaning this one contains the real id.

        let mut user_id: Option<u32> = None;
        for (defined_route_fragment, real_route_fragment) in request
            .route()
            .unwrap()
            .uri
            .segments()
            .zip(request.uri().segments())
        {
            if defined_route_fragment == "<user_id>" || defined_route_fragment == "<_user_id>" {
                // its "ok" to use `unwrap()` here because the `user_id` MUST be declared as a u32
                // in the function definition
                // but its better to manage it with a `match`
                user_id = Some(real_route_fragment.parse::<u32>().unwrap());
                break;
            }
        }
        match user_id {
            Some(id) => match UserEntity::by_id(&conn, &id).unwrap() {
                Some(user) => Outcome::Success(Self { entity: user }),
                None => Outcome::Failure((
                    Status::BadRequest,
                    Error::EntityError(EntityError::InvalidID),
                )),
            },
            None => Outcome::Forward(()),
        }
    }
}
