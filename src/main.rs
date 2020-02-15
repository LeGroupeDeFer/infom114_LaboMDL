#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::{Flash, Redirect};

// #[macro_use]
static DATABASE_URL: &'static str = "mysql://testuser:testpassword@localhost/test_rocket";
use rocket_contrib::templates::Template;

use models::user::User;

#[database("mariadb_test")]
struct MyDbConn(diesel::MysqlConnection);

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[derive(Serialize)]
struct LoginTemplateContext {
    title: &'static str,
    parent: &'static str,
}

#[derive(Serialize)]
struct UserInfoTemplateContext<'a> {
    title: &'static str,
    user: &'a User,
    parent: &'static str,
}

#[derive(FromForm)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

pub struct Authentification(User);

const AUTH_COOKIE: &'static str = "log_id";

impl<'a, 'r> FromRequest<'a, 'r> for Authentification {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.cookies().get_private(AUTH_COOKIE) {
            Some(cookie) => {
                let value = cookie.value().parse::<i32>().unwrap();
                let user = User::from(&value);
                match user {
                    Some(u) => rocket::Outcome::Success(Authentification(u)),
                    None => rocket::Outcome::Forward(()),
                }
            }
            None => rocket::Outcome::Forward(()),
        }
    }
}

impl Authentification {
    pub fn login(cookies: &mut Cookies, user: &User) {
        let auth_cookie = Cookie::new(AUTH_COOKIE, user.cookie());
        cookies.add_private(auth_cookie);
    }

    pub fn logout(cookies: &mut Cookies) {
        cookies.remove_private(Cookie::named(AUTH_COOKIE));
    }
}

#[get("/test?<option>")]
fn optional(option: Option<u32>) -> &'static str {
    match option {
        Some(_n) => "Oh ya un nombre",
        None => "rien à voir ici",
    }
}

#[get("/cookies")]
fn get_cookies(cookies: Cookies) -> String {
    cookies
        .iter()
        .map(|c| format!("name : {:?}, value : {:?}", c.name(), c.value()))
        .collect::<String>()
}

#[get("/")]
fn index(conn: MyDbConn) -> String {
    // use schema::users::dsl::*;

    let results = schema::users::dsl::users
        // .filter(true)
        .limit(5)
        .load::<User>(&*conn)
        .expect("error loading post");

    format!("{:?}", results)
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
                .filter(schema::users::dsl::username.eq(&info.username))
                .limit(1)
                .load::<User>(&*conn)
                .expect("user is not in db");

            if users.len() < 1 {
                return Flash::error(Redirect::to("/login"), "User not in DB");
            }

            let user = &users[0];
            if user.password == info.password {
                Authentification::login(&mut cookies, &user);

                Flash::success(Redirect::to("/hidden"), "Login successfull")
            } else {
                Flash::error(Redirect::to("/login"), "Wrong password")
            }
        }
    }
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    Authentification::logout(&mut cookies);
    return Flash::success(Redirect::to("/login"), "Successfully logout");
}

#[get("/hidden")]
fn hidden(auth: Authentification) -> Template {
    Template::render(
        "hidden",
        &UserInfoTemplateContext {
            title: "Hidden",
            user: &auth.0,
            parent: "layout",
        },
    )
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
            parent: "layout",
        },
    )
}

/// Serve static files
// #[get("/<file..>")]
// fn files(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).ok()
// }

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                optional,
                hello,
                login,
                post_login,
                hidden,
                get_cookies,
                logout,
            ],
        )
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
        .launch();
}
