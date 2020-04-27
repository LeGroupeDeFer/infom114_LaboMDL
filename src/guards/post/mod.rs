use crate::database;
use crate::database::models::prelude::*;
use diesel::MysqlConnection;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostGuard {
    entity: Post,
}

impl PostGuard {
    pub fn post(&self) -> &Post {
        &self.entity
    }

    pub fn post_clone(self) -> Post {
        self.entity
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PostGuard {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn: MysqlConnection = database::connection(&database::url());

        // `request.route().unwrap().uri` is the route that we've defined with a macro
        // meaning that this one contains the string `<post_id>´

        // `request.uri()` is the route that the router got from the client
        // meaning this one contains the real id.

        let mut post_id: Option<u32> = None;
        for (defined_route_fragment, real_route_fragment) in request
            .route()
            .unwrap()
            .uri
            .segments()
            .zip(request.uri().segments())
        {
            if defined_route_fragment == "<post_id>" || defined_route_fragment == "<_post_id>" {
                // its "ok" to use `unwrap()` here because the `post_id` MUST be declared as a u32
                // in the function definition
                // but its better to manage it with a `match`
                post_id = Some(real_route_fragment.parse::<u32>().unwrap());
                break;
            }
        }
        match post_id {
            Some(id) => match Post::by_id(&conn, &id).unwrap() {
                Some(post) => Outcome::Success(Self { entity: post }),
                None => Outcome::Failure((Status::BadRequest, "Invalid ID supplied".to_string())),
            },
            None => Outcome::Forward(()),
        }
    }
}
