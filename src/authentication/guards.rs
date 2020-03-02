use crate::models::user::User;

use super::AUTH_COOKIE;

use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{FromRequest, Outcome, Request};

pub struct Auth {
    pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(AUTH_COOKIE) {
            Some(cookie) => {
                let value = cookie.value().parse::<u32>().unwrap();
                let user = User::from(&value);
                match user {
                    Some(u) => {
                        // if u.has_right :
                        rocket::Outcome::Success(Auth { user: u })
                        // else :
                        // Outcome::Failure((Status::Unauthorized, "Missing capability".to_string()))
                    }
                    None => Outcome::Failure((Status::Forbidden, "User not found".to_string())),
                }
            }
            None => Outcome::Failure((Status::Forbidden, "Authentification required".to_string())),
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

    pub fn is_authenticated(cookies: &mut Cookies) -> bool {
        match cookies.get_private(AUTH_COOKIE) {
            Some(cookie) => {
                let value = cookie.value().parse::<u32>().unwrap();
                match User::from(&value) {
                    Some(_) => true,
                    None => false,
                }
            }
            None => false,
        }
    }
}
