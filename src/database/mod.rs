//! # Database
//!
//! This module aims to group everything that is related to the database.
pub mod models;
pub mod schema;

// ---------------- REQUIRES --------------------------------------------------

use rocket::config::Value;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;
use std::env;

use rocket::config::Value;

use dotenv::dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::conf::env_setting;

// --------------------- DB OBJECT --------------------------------------------

#[database("mariadb_pool")]
pub struct Connection(diesel::MysqlConnection);

// --------------------- FUNCTIONS --------------------------------------------

pub fn pool(database_url: &str) -> HashMap<&str, Value> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(database_url));
    databases.insert("mariadb_pool", Value::from(database_config));

    databases
}

// --------------------- FUNCTIONS --------------------------------------------

/// Get the database url set in the `.env` file.
///
/// The `cfg()` tricks allows us to use a different database for live & test.
#[cfg(not(test))]
pub fn url() -> String {
    dotenv().ok();

    // DB settings
    let db_host = env_setting("DB_HOST");
    let db_port = env_setting("DB_PORT");
    let db_adapter = env_setting("DB_ADAPTER");
    let db_user = env_setting("DB_USER");
    let db_password = env_setting("DB_PASSWORD");
    let db_database = env_setting("DB_DATABASE");

    // DB url
    format!(
        "{}://{}:{}@{}:{}/{}",
        db_adapter, db_user, db_password, db_host, db_port, db_database
    )
}

#[cfg(test)]
pub fn url() -> String {
    dotenv().ok();

    // DB settings
    let db_host = env_setting("TEST_DB_HOST");
    let db_port = env_setting("TEST_DB_PORT");
    let db_adapter = env_setting("TEST_DB_ADAPTER");
    let db_user = env_setting("TEST_DB_USER");
    let db_password = env_setting("TEST_DB_PASSWORD");
    let db_database = env_setting("TEST_DB_DATABASE");

    // DB url
    format!(
        "{}://{}:{}@{}:{}/{}",
        db_adapter, db_user, db_password, db_host, db_port, db_database
    )
}

/// Get a database connection.
///
/// Often used when the operations are not derived from a `Rocket::Request`.
/// If it's possible, always prefer the use of the `MyDbConn` which can be
/// retrieved from a request.
pub fn connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = url();
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
