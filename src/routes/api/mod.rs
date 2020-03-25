use crate::lib::extend_routes;
use rocket;
mod v1;

pub fn collect() -> Vec<rocket::Route> {
    [&v1::collect()[..], &extend_routes("/v1", v1::collect())[..]].concat()
}
