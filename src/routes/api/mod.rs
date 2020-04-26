use std::path::PathBuf;

use regex::Regex;
use rocket::response::Redirect;
use rocket;

use crate::database::models::prelude::*;

mod v1;

pub fn collect() -> Vec<rocket::Route> {
    [
        &v1::collect()[..],
        &routes!(api_get, api_post, api_put, api_delete)[..]
    ]
    .concat()
}

pub fn api(path: PathBuf) -> Result<Redirect> {
    let components: Vec<&str> = path.as_path()
        .components()
        .map(|c| c.as_os_str().to_str().unwrap()) // FIXME - Remove unwrap
        .collect();

    let api_version = Regex::new(r"v[0-9]+").unwrap();
    if api_version.is_match(components[0]) {
        // If the version was specified and yet this route is called, we've got a 404
        Err(Error::NotFound)?
    }

    let uri: String = components.iter().fold(
        String::from("/api/v1"),
        |acc, comp| format!("{}/{}", acc, comp)
    );

    // We need to return a 307 to retain the HTTP method & payload
    Ok(Redirect::temporary(uri))
}

#[get("/api/<path..>", format="json", rank=6)]
pub fn api_get(path: PathBuf) -> Result<Redirect> { api(path) }

#[post("/api/<path..>", format="json", rank=6)]
pub fn api_post(path: PathBuf) -> Result<Redirect> { api(path) }

#[put("/api/<path..>", format="json", rank=6)]
pub fn api_put(path: PathBuf) -> Result<Redirect> { api(path) }

#[delete("/api/<path..>", format="json", rank=6)]
pub fn api_delete(path: PathBuf) -> Result<Redirect> { api(path) }
