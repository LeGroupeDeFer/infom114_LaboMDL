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

use dotenv::dotenv;
use rocket_contrib::templates::Template;

/* ----------------------------- Local modules ----------------------------- */

mod auth;
mod authentication;
mod conf;
mod database;
mod http;
mod lib;
mod routes;

/* ----------------------------- Launch Rocket ----------------------------- */

fn main() {
    dotenv().ok();
    rocket::custom(conf::from_env())
        .attach(conf::AppState::manage())
        .attach(database::Connection::fairing())
        .attach(Template::fairing())
        .register(http::errors::catchers::collect())
        .mount("/", routes::collect())
        .launch();
}
