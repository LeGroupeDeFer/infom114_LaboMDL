use crate::database::MyDbConn;
use crate::models::user::User;
use crate::schema;

use super::forms::LoginCredentials;
use super::guards::Auth;
use super::templates::LoginTemplateContext;

use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

use diesel::prelude::*;

pub fn collect() -> Vec<rocket::Route> {
    routes!(login, post_login, logout)
}

#[get("/login")]
fn login() -> Template {
    Template::render(
        "login",
        &LoginTemplateContext {
            title: "Login",
            parent: "layout",
        },
    )
}

#[post("/login", data = "<credentials>")]
fn post_login(
    credentials: Option<Form<LoginCredentials>>,
    conn: MyDbConn,
    mut cookies: Cookies,
) -> Flash<Redirect> {
    match credentials {
        None => Flash::error(Redirect::to("/login"), "Could not retreive credentials"),
        Some(info) => {
            let users = schema::users::dsl::users
                .filter(schema::users::columns::username.eq(&info.username))
                .limit(1)
                .load::<User>(&*conn)
                .expect("user is not in db");

            if users.len() < 1 {
                return Flash::error(Redirect::to("/login"), "User not in DB");
            }

            let user = &users[0];
            if user.password == info.password {
                Auth::login(&mut cookies, &user);

                Flash::success(Redirect::to("/hidden"), "Login successfull")
            } else {
                Flash::error(Redirect::to("/login"), "Wrong password")
            }
        }
    }
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    Auth::logout(&mut cookies);
    return Flash::success(Redirect::to("/login"), "Successfully logout");
}

#[cfg(test)]
mod test {

    #[test]
    fn test_collect() {
        // its ugly af to compare strings output
        // quicly implemented like this to test code cov
        assert_eq!(
            format!(
                "{:?}",
                routes!(super::login, super::post_login, super::logout)
            ),
            format!("{:?}", super::collect())
        );
    }
}
