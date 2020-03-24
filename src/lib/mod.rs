use rocket::http::uri;

pub mod mail;

pub fn set_uri(prefix: &str, route: &rocket::Route) -> rocket::Route {
    let base = uri::Origin::parse(prefix).unwrap();
    let route_uri = route.uri.clone();
    let mut new_route = route.clone();
    if let Err(error) = new_route.set_uri(base, route_uri) {
        panic!(error);
    }
    new_route
}

pub fn extend_routes(prefix: &str, routes: Vec<rocket::Route>) -> Vec<rocket::Route> {
    routes.iter().map(|route| set_uri(prefix, route)).collect()
}
