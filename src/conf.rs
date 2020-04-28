use crate::database;
use rocket::config::{Config, Environment};
use rocket::fairing::AdHoc;
use std::env;

pub type State<'a> = rocket::State<'a, AppState>;

// Add app-wide variables here!
pub struct AppState {
    pub jwt_secret: Vec<u8>,
    pub access_lifetime: u32,
    pub refresh_lifetime: u32
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_attach("Application state", |rocket| {
            // Using the env here as to palliate to "rocket" borrowing...
            let secret = env_setting("JWT_SECRET");
            let access_lifetime = env_setting("JWT_LIFETIME")
                .parse::<u32>()
                .expect("JWT_LIFETIME must be a natural");
            let refresh_lifetime = env_setting("REFRESH_LIFETIME")
                .parse::<u32>()
                .expect("REFRESH_LIFETIME must be a natural");

            Ok(rocket.manage(AppState {
                jwt_secret: secret.as_bytes().to_vec(),
                access_lifetime,
                refresh_lifetime,
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
