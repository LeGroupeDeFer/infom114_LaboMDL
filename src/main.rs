#![feature(proc_macro_hygiene, decl_macro)]

/* --------------------------- Load Extern Crates -------------------------- */

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

/* ------------------------------ Local Mod -------------------------------- */

mod authentication;
mod database;
mod http;
mod models;
mod schema;

/* --------------------------- Load Namespaces ----------------------------- */

use std::path::{Path, PathBuf};

use database::MyDbConn;

use rocket::http::Cookies;
use rocket::response::NamedFile;

use rocket_contrib::templates::Template;

/* -------------------------------- Routes --------------------------------- */

/// Serve application entrypoint
#[get("/")]
fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some routes to fetch entrypoint when refreshed
#[get("/<route>", rank = 10)]
fn dynamic_routing(route: String) -> Option<Template> {
    let mut allowed_routes = vec!["profile", "notifications", "settings", "about"];

    allowed_routes.append(&mut authentication::routes::allowed_paths());

    if allowed_routes.contains(&&route[..]) {
        Some(Template::render("layout", &()))
    } else {
        None
    }
}

/// Serve static files
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// Display cookies stored
// TODO : remove me when done testing
#[get("/dev/cookies")]
fn get_cookies(cookies: Cookies) -> String {
    cookies
        .iter()
        .map(|c| format!("name : {:?}, value : {:?}", c.name(), c.value()))
        .collect::<String>()
}

/* ----------------------------- Launch Rocket ----------------------------- */

fn main() {
    rocket::ignite()
        .mount("/", routes![index, dynamic_routing, files, get_cookies])
        .mount("/", authentication::routes::collect())
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
        .launch();
}
