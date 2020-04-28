use regex::Regex;
use rocket;
use rocket::response::Redirect;
use std::path::PathBuf;

use crate::lib::consequence::*;

mod v1;

pub fn collect() -> Vec<rocket::Route> {
    [
        &v1::collect()[..],
        &routes!(api_get, api_post, api_put, api_delete)[..],
    ]
    .concat()
}

pub fn api(path: PathBuf) -> Consequence<Redirect> {
    let components: Vec<&str> = path
        .as_path()
        .components()
        .map(|c| c.as_os_str().to_str().unwrap()) // FIXME - Remove unwrap
        .collect();

    let api_version = Regex::new(r"v[0-9]+").unwrap();
    if api_version.is_match(components[0]) {
        // If the version was specified and yet this route is called, we've got a 404
        Err(Error::NotFound)?
    }

    let uri: String = components
        .iter()
        .fold(String::from("/api/v1"), |acc, comp| {
            format!("{}/{}", acc, comp)
        });

    // We need to return a 307 to retain the HTTP method & payload
    Ok(Redirect::temporary(uri))
}

#[get("/api/<path..>", format = "json", rank = 10)]
pub fn api_get(path: PathBuf) -> Consequence<Redirect> {
    api(path)
}

#[post("/api/<path..>", format = "json", rank = 2)]
pub fn api_post(path: PathBuf) -> Consequence<Redirect> {
    api(path)
}

#[put("/api/<path..>", format = "json", rank = 2)]
pub fn api_put(path: PathBuf) -> Consequence<Redirect> {
    api(path)
}

#[delete("/api/<path..>", format = "json", rank = 2)]
pub fn api_delete(path: PathBuf) -> Consequence<Redirect> {
    api(path)
}
