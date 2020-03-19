// ---------------- REQUIRES --------------------------------------------------

use std::collections::HashMap;
use std::env;

use rocket::config::{Config, Environment, Value};

use dotenv::dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;

// ------------------ CONST ---------------------------------------------------

const PROD_DB_URL: &'static str = "DATABASE_URL";
const TEST_DB_URL: &'static str = "TEST_DATABASE_URL";

// --------------------- DB OBJECT --------------------------------------------

#[database("mariadb_unanimity")]
pub struct MyDbConn(diesel::MysqlConnection);

// --------------------- FUNCTIONS --------------------------------------------

#[cfg(not(test))]
fn url() -> String {
    dotenv().ok();
    env::var(PROD_DB_URL).expect(&format!("{} must be set", PROD_DB_URL))
}

#[cfg(test)]
fn url() -> String {
    dotenv().ok();
    env::var(TEST_DB_URL).expect(&format!("{} must be set", TEST_DB_URL))
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
    database_config.insert("url", Value::from(url().as_ref()));
    databases.insert("mariadb_unanimity", database_config);

    Config::build(Environment::Development)
        .address("0.0.0.0")
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
