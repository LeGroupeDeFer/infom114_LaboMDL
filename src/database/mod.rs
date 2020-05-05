//! # Database
//!
//! This module aims to group everything that is related to the database.
pub mod models;
pub mod schema;
pub mod tables;

// ---------------- REQUIRES --------------------------------------------------

use crate::conf::{env_setting, env_setting_or};
use crate::diesel::Connection;
use crate::lib::EntityError;
use diesel::MysqlConnection;
use dotenv::dotenv;
use rocket::config::Value;
use rocket_contrib::databases::diesel;
use serde::export::TryFrom;
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

    let app_env = &*env_setting_or("MODE", "DEV".into());

    let mut db_uri: String;
    if vec!("DEV", "DEVELOPMENT", "PROD", "PRODUCTION").iter().any(|&m| m == app_env) {
        db_uri = format!(
            "mysql://{}:{}@{}:{}/{}",
            env_setting("DB_USER"),
            env_setting("DB_PASSWORD"),
            env_setting("DB_HOST"),
            env_setting("DB_PORT"),
            env_setting("DB_DATABASE")
        );
    } else {
        db_uri = format!(
            "mysql://{}:{}@{}:{}/{}",
            env_setting("TEST_DB_USER"),
            env_setting("TEST_DB_PASSWORD"),
            env_setting("TEST_DB_HOST"),
            env_setting("TEST_DB_PORT"),
            env_setting("TEST_DB_DATABASE")
        );
    }

    println!("TARGET DATABASE ({}): {}", app_env, db_uri);
    db_uri
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

pub enum SortOrder {
    New,
    Old,
    HighScore,
    LowScore,
    HighRank,
    LowRank
}

impl TryFrom<&str> for SortOrder {
    type Error = EntityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "new" => Ok(Self::New),
            "old" => Ok(Self::Old),
            "top" => Ok(Self::HighScore),
            "low" => Ok(Self::LowScore),
            "high_rank" => Ok(Self::HighRank),
            "low_rank" => Ok(Self::LowRank),
            _ => Err(EntityError::InvalidAttribute),
        }
    }
}
