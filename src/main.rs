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

mod authentication;
mod database;
mod models;
mod schema;

use std::path::{Path, PathBuf};

use authentication::guards::Auth;
use database::MyDbConn;
use models::user::User;

use rocket::http::Cookies;
use rocket::response::NamedFile;

use rocket_contrib::templates::Template;

use diesel::prelude::*;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[derive(Serialize)]
struct UserInfoTemplateContext<'a> {
    title: &'static str,
    user: &'a User,
    parent: &'static str,
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
    let results = schema::users::dsl::users
        // .filter(true)
        .limit(5)
        .load::<User>(&*conn)
        .expect("error loading post");

    format!("{:?}", results)
}

#[get("/hidden")]
fn hidden(auth: Auth) -> Template {
    Template::render(
        "hidden",
        &UserInfoTemplateContext {
            title: "Hidden",
            user: &auth.user,
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
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![files, index, optional, hello, hidden, get_cookies],
        )
        .mount("/", authentication::routes::mount())
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
        .launch();
}
