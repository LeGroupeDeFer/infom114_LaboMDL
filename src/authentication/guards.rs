use super::AUTH_COOKIE;
use crate::conf::AppState;
use crate::database::models::user::User;

use diesel::prelude::MysqlConnection;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

pub struct Auth {
    pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let state = request.guard::<State<AppState>>().unwrap();
        let conn = request.guard::
        let secret = state.jwt_secret;

        request
            .cookies()
            .get_private(AUTH_COOKIE)
            .ok_or("Authentification required".to_string())
            .map(|cookie| cookie.value().parse::<u32>().unwrap())
            .map(|id| User::from(&conn, id))
            .and_then(|user| user.ok_or("User not found".to_string()))
            .map(|user| Auth { user: user })
            .map_or_else(
                |e| Outcome::Failure((Status::Forbidden, e)),
                Outcome::Success,
            )
    }
}

impl Auth {
    pub fn login(cookies: &mut Cookies, user: &User) {
        let auth_cookie = Cookie::new(AUTH_COOKIE, user.cookie());
        cookies.add_private(auth_cookie);
    }

    pub fn logout(cookies: &mut Cookies) {
        cookies.remove_private(Cookie::named(AUTH_COOKIE));
    }

    pub fn is_authenticated(conn: &MysqlConnection, cookies: &Cookies) -> bool {
        cookies
            .get_private(AUTH_COOKIE)
            .map(|cookie| cookie.value().parse::<u32>().unwrap())
            .map(|id| User::from(conn, id))
            .is_some()
    }
}
