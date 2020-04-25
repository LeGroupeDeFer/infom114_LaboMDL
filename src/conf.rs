use crate::database;
use rocket::config::{Config, Environment};
use rocket::fairing::AdHoc;
use std::env;

pub type State<'a> = rocket::State<'a, AppState>;

// Add app-wide variables here!
pub struct AppState {
    pub jwt_secret: Vec<u8>,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_attach("Application state", |rocket| {
            // Using the env here as to palliate to "rocket" borrowing...
            let secret = env_setting("JWT_SECRET");
            Ok(rocket.manage(AppState {
                jwt_secret: secret.as_bytes().to_vec(),
            }))
        })
    }
}

pub fn env_setting(key: &str) -> String {
    dotenv::dotenv().ok();
    env::var(key)
        .ok()
        .map(|v| v)
        .expect(&format!("Unbound variable {}", key))
}

pub fn from_env() -> Config {
    let environment = Environment::active().expect("Unknown environment");
    let interface = env_setting("INTERFACE");
    let port = env_setting("PORT")
        .parse::<u16>()
        .expect("PORT: Expected integer");

    let database_url = database::url();

    let database_pool = database::pool(&database_url);

    Config::build(environment)
        .environment(environment)
        .address(interface)
        .port(port)
        .extra("databases", database_pool)
        .finalize()
        .unwrap()
}
