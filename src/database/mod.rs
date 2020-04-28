//! # Database
//!
//! This module aims to group everything that is related to the database.
pub mod models;
pub mod schema;
pub mod tables;

// ---------------- REQUIRES --------------------------------------------------

use crate::conf::env_setting;
use crate::diesel::Connection;
use diesel::MysqlConnection;
use dotenv::dotenv;
use rocket::config::Value;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;

// --------------------- DB OBJECT --------------------------------------------

#[database("mariadb_pool")]
pub struct DBConnection(diesel::MysqlConnection);

// --------------------- FUNCTIONS --------------------------------------------

/// Create a HashMap that can be used to feed a `Rocket::Config` object with
/// database related information, such as the url of the used database.
///
/// The application database shared name is `mariadb_pool`.
pub fn pool(database_url: &str) -> HashMap<&str, Value> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(database_url));
    databases.insert("mariadb_pool", Value::from(database_config));

    databases
}

// --------------------- FUNCTIONS --------------------------------------------

/// Get the database url set in the `.env` file.
pub fn url() -> String {
    dotenv().ok();

    // DB settings
    let db_adapter = "mysql"; // imposed by the use of MysqlConnection type
    let db_host = env_setting("DB_HOST");
    let db_port = env_setting("DB_PORT");
    let db_user = env_setting("DB_USER");
    let db_password = env_setting("DB_PASSWORD");
    let db_database = env_setting("DB_DATABASE");

    // DB url
    format!(
        "{}://{}:{}@{}:{}/{}",
        db_adapter, db_user, db_password, db_host, db_port, db_database
    )
}

/// Get a database connection.
///
/// Often used when the operations are not derived from a `Rocket::Request`.
/// If it's possible, always prefer the use of the `DBConnection` which can be
/// retrieved from a request.
pub fn connection(database_url: &str) -> MysqlConnection {
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Data
///
/// This is a sementic wrapper so a caller is able to know the new database
/// state simply by retreiving data wrapped inside it
pub enum Data<T> {
    Existing(T),
    Inserted(T),
    Updated(T),
}
