//! # UNanimity Library
//!
//! The crate `unanimity` is a work in progress website that aims to provide a
//! participative platform for the UNamur members.
//!
//! The library crate allow us to perform some documentation tests.

// unstable features used by rocket
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
extern crate regex;

/* ------------------------------ Local Mod -------------------------------- */

pub mod authentication;
pub mod database;
mod http;
pub mod models;
pub mod schema;

/* --------------------------- Load Namespaces ----------------------------- */

use std::path::{Path, PathBuf};

use rocket::config::{Config, Environment};
use rocket::response::NamedFile;
use rocket::Rocket;

use rocket_contrib::templates::Template;

use database::{db_config, MyDbConn};

/* -------------------------------- Routes --------------------------------- */

/// Serve application entrypoint
///
/// The React app takes the lead to manage the front-end
#[get("/")]
fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some route to return a 200 ok with the react layout template
///
/// Since the client navigation is managed by react, when a client forces the
/// refresh of a page, the backend returns a HTTP 404.
/// This `dynamic_routing` is design to prevent that from happening
#[get("/<route>", rank = 2)]
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
///
/// Every `js`, `css` or image file found in the `/static/` folder is served
/// with this route.
#[get("/<file..>", rank = 3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/* ----------------------------- Ignite Rocket ----------------------------- */

/// Prepare the Rocket app
pub fn rocket() -> Rocket {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .extra("databases", db_config())
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index, dynamic_routing, files])
        .mount("/", authentication::routes::collect())
        .register(http::errors::catchers::collect())
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
}
