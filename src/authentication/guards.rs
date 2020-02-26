use crate::models::user::User;

use super::AUTH_COOKIE;

use rocket::http::{Cookie, Cookies};
use rocket::request::{FromRequest, Outcome, Request};

pub struct Auth {
    pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(AUTH_COOKIE) {
            Some(cookie) => {
                let value = cookie.value().parse::<u32>().unwrap();
                let user = User::from(&value);
                match user {
                    Some(u) => rocket::Outcome::Success(Auth { user: u }),
                    None => rocket::Outcome::Forward(()),
                }
            }
            None => rocket::Outcome::Forward(()),
        }
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
}
