pub mod models;
pub mod schema;

// ---------------- REQUIRES --------------------------------------------------

use rocket::config::Value;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;

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
