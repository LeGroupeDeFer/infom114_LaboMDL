use crate::database;
use crate::database::models::prelude::*;
use crate::lib::{EntityError, Error};
use diesel::MysqlConnection;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request};

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleGuard {
    entity: RoleEntity,
}

impl RoleGuard {
    pub fn role(&self) -> &RoleEntity {
        &self.entity
    }

    pub fn role_clone(self) -> RoleEntity {
        self.entity
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RoleGuard {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn: MysqlConnection = database::connection(&database::url());

        // `request.route().unwrap().uri` is the route that we've defined with a macro
        // meaning that this one contains the string `<role_id>´

        // `request.uri()` is the route that the router got from the client
        // meaning this one contains the real id.

        let mut role_id: Option<u32> = None;
        for (defined_route_fragment, real_route_fragment) in request
            .route()
            .unwrap()
            .uri
            .segments()
            .zip(request.uri().segments())
        {
            if defined_route_fragment == "<role_id>" || defined_route_fragment == "<_role_id>" {
                // its "ok" to use `unwrap()` here because the `role_id` MUST be declared as a u32
                // in the function definition
                // but its better to manage it with a `match`
                role_id = Some(real_route_fragment.parse::<u32>().unwrap());
                break;
            }
        }
        match role_id {
            Some(id) => match RoleEntity::by_id(&conn, &id).unwrap() {
                Some(role) => Outcome::Success(Self { entity: role }),
                None => Outcome::Failure((
                    Status::BadRequest,
                    Error::EntityError(EntityError::InvalidID),
                )),
            },
            None => Outcome::Forward(()),
        }
    }
}
