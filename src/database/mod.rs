//! # Database
//!
//! This module aims to group everything that is related to the database.

// ---------------- REQUIRES --------------------------------------------------

use std::collections::HashMap;
use std::env;

use rocket::config::Value;

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

/// Get the database url set in the `.env` file.
///
/// The `cfg()` tricks allows us to use a different database for live & test.
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

/// Create a `HashMap` that can be used in `rocket::config::Config` to setup a
/// database connection.
///
/// This trick allows us to avoid the need of a `Rocket.toml` file, and to
/// use a `.env` configuration file.
pub fn db_config() -> HashMap<&'static str, HashMap<&'static str, Value>> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(url().as_ref()));
    databases.insert("mariadb_unanimity", database_config);

    databases
}
