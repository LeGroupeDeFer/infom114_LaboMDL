//! # UNanimity Library
//!
//! The crate `unanimity` is a work in progress website that aims to provide a
//! participative platform for the UNamur members.
//!
//! The library crate allow us to perform some documentation tests.

// unstable features used by rocket
#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

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

use rocket::Rocket;
use rocket_contrib::templates::Template;

/* ----------------------------- Local modules ----------------------------- */

pub mod auth;
pub mod conf;
pub mod database;
pub mod http;
mod lib;
mod routes;
// pub mod schema;

/* ----------------------------- Ignite Rocket ----------------------------- */

/// Prepare the Rocket app
pub fn rocket(ignited: Rocket) -> Rocket {
    ignited
        .attach(conf::AppState::manage())
        .attach(database::DBConnection::fairing())
        .attach(Template::fairing())
        .register(http::errors::catchers::collect())
        .mount("/", routes::collect())
}

/// Load the configuration from the `.env` file
/// Returns a basic `Rocket` object
pub fn ignite() -> Rocket {
    rocket::custom(conf::from_env())
}
