//! # UNanimity Library
//!
//! The crate `unanimity` is a work in progress website that aims to provide a
//! participative platform for the UNamur members.

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
mod schema;

/* --------------------------- Load Namespaces ----------------------------- */

use std::path::{Path, PathBuf};

use rocket::config::Config;
use rocket::response::NamedFile;
use rocket::Rocket;

use rocket_contrib::templates::Template;

use database::{db_config, MyDbConn};

/* -------------------------------- Routes --------------------------------- */

/// Serve application entrypoint
#[get("/")]
fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some routes to fetch entrypoint when refreshed
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
#[get("/<file..>", rank = 3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/* ----------------------------- Ignite Rocket ----------------------------- */

///
pub fn rocket() -> Rocket {
    rocket::custom(db_config())
        .mount("/", routes![index, dynamic_routing, files])
        .mount("/", authentication::routes::collect())
        .register(http::errors::catchers::collect())
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
}
