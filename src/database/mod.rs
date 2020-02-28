// ---------------- REQUIRES --------------------------------------------------

use std::collections::HashMap;
use std::env;

use rocket::config::{Config, Environment, Value};

use dotenv::dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;

// ------------------ CONST ---------------------------------------------------

const ENV_DATABASE_URL: &'static str = "DATABASE_URL";

// --------------------- DB OBJECT --------------------------------------------

#[database("mariadb_test")]
pub struct MyDbConn(diesel::MysqlConnection);

// --------------------- FUNCTIONS --------------------------------------------

fn url() -> String {
    dotenv().ok();
    env::var(ENV_DATABASE_URL).expect(&format!("{} must be set", ENV_DATABASE_URL))
}

pub fn connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = url();
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn db_config() -> Config {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(&url()[..]));
    databases.insert("mariadb_test", database_config);

    Config::build(Environment::Development)
        .address("0.0.0.0")
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
