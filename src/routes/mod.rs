mod api;
use crate::lib::extend_routes;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

pub fn collect() -> Vec<rocket::Route> {
    [
        &routes!(index, dynamic_routing, files, activate, recover)[..],
        &extend_routes("/api", api::collect())[..],
    ]
    .concat()
}

const ALLOWED_ROUTES: [&str; 9] = [
    "profile",
    "notifications",
    "settings",
    "about",
    "login",
    "logout",
    "register",
    "recovery",
    "activate",
];

// /api/<...subpath> => /api/v<version>/<...subpath>

/* --------------------------------- Routes -------------------------------- */

/// Serve application entrypoint
#[get("/")]
pub fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some routes to fetch entrypoint when refreshed
#[get("/<route>", rank = 2)]
pub fn dynamic_routing(route: String) -> Option<Template> {
    if ALLOWED_ROUTES.contains(&&route[..]) {
        Some(Template::render("layout", &()))
    } else {
        None
    }
}

#[get("/activate/<id>/<token>", rank = 1)]
pub fn activate(id: u32, token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

#[get("/recover/<id>/<token>", rank = 1)]
pub fn recover(id: u32, token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

/// Serve static files
#[get("/<file..>", rank = 3)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
