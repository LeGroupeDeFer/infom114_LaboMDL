use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[database("mariadb_test")]
pub struct MyDbConn(diesel::MysqlConnection);
