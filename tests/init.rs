//! # Test init
//!
//! Initialisations and helpers to ease the developpment of automated tests.
//!
//! the test database MUST be availlable
//! the migrations MUST have been applied to the test database

use unanimitylibrary::conf::env_setting;

use unanimitylibrary::database;
use unanimitylibrary::database::models::{Entity, address::Address, user::User, user::UserMinima};
use unanimitylibrary::database::schema::addresses::dsl::addresses;
use unanimitylibrary::database::schema::users::dsl::users;

use diesel::query_dsl::RunQueryDsl;


/// Truncate all the tables
pub fn clean() {
    // get connection
    let conn = database_connection();

    // truncate all tables
    diesel::delete(users).execute(&conn).unwrap();
    diesel::delete(addresses).execute(&conn).unwrap();

    // assert empty database
    assert_eq!(users.load::<User>(&conn).unwrap().len(), 0);
    assert_eq!(addresses.load::<Address>(&conn).unwrap().len(), 0);
}

pub fn database_connection() -> diesel::MysqlConnection {
    database::connection(&database_url())
}

/// Get a client that can be used to perform some HTTP actions on the
/// Rocket routes of the unanimity application
pub fn client() -> rocket::local::Client {
    // get Rocket instance
    let rocket = unanimitylibrary::rocket(ignite());

    // return new Client
    rocket::local::Client::new(rocket).expect("valid rocket instance")
}

/// Truncate the database & get a rocket client
pub fn clean_client() -> rocket::local::Client {
    // init & get client
    clean();
    client()
}

pub fn database_url() -> String {
    dotenv::dotenv().ok();

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

pub fn ignite() -> rocket::Rocket {
    let environment = rocket::config::Environment::active().expect("Unknown environment");
    let interface = env_setting("INTERFACE");
    let port = env_setting("PORT")
        .parse::<u16>()
        .expect("PORT: Expected integer");

    let database_url = database_url();

    let database_pool = database::pool(&database_url);

    let config = rocket::Config::build(environment)
        .environment(environment)
        .address(interface)
        .port(port)
        .extra("databases", database_pool)
        .finalize()
        .unwrap();

    rocket::custom(config)
}

pub fn get_user(active: bool) -> (User, String) {
    let conn = database_connection();

    let u = UserMinima {
        email: String::from("guillaume.latour@student.unamur.be"),
        password: String::from("mysuperpassword"),
        firstname: String::from("Guillaume"),
        lastname: String::from("Latour"),
        address: None,
        phone: None,
        activation_token: None,
        recovery_token: None,
        refresh_token: None,
    };

    let user = User::insert_either(&conn, &u).unwrap();

    if active {
        user.activate(&conn).unwrap();
    }

    (user, u.password)
}
