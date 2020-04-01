//! # Routes
//!
//! Group all routes of the application, classified by mod respecting the
//! route's path.
//!
//! Some of the routes are just hollow routes that just returns the basic
//! application layout.
//!
//! This is needed since the front is managed by React.

mod api;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

pub fn collect() -> Vec<rocket::Route> {
    [
        &routes!(index, dynamic_routing, files, activate, recover)[..],
        &api::collect()[..],
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
///
/// The React app takes the lead to manage the front-end
#[get("/")]
pub fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some route to return a 200 ok with the react layout template
///
/// Since the client navigation is managed by react, when a client forces the
/// refresh of a page, the backend returns a HTTP 404.
/// This `dynamic_routing` is design to prevent that from happening
#[get("/<route>", rank = 2)]
pub fn dynamic_routing(route: String) -> Option<Template> {
    if ALLOWED_ROUTES.contains(&&route[..]) {
        Some(Template::render("layout", &()))
    } else {
        None
    }
}

/// Hollow route to be accessed by activation link
#[get("/activate/<_id>/<_token>", rank = 1)]
pub fn activate(_id: u32, _token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

/// Hollow route to be accessed by recovery link
#[get("/recover/<_id>/<_token>", rank = 1)]
pub fn recover(_id: u32, _token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

/// Serve static files
///
/// Every `js`, `css` or image file found in the `/static/` folder is served
/// with this route.
#[get("/<file..>", rank = 3)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
