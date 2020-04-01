//! # Test init
//!
//! Initialisations and helpers to ease the developpment of automated tests.
//!
//! the test database MUST be availlable
//! the migrations MUST have been applied to the test database

use unanimitylibrary::conf::env_setting;

use unanimitylibrary::database;
use unanimitylibrary::database::models::{
    address::Address,
    roles::{capability::Capability, role::Role, role_capability::RelRoleCapability},
    user::{User, UserMinima},
};

use unanimitylibrary::database::schema::{
    addresses, capabilities, roles, roles_capabilities, users, users_roles,
};

use unanimitylibrary::database::schema::addresses::dsl::addresses as addresses_table;
use unanimitylibrary::database::schema::capabilities::dsl::capabilities as capabilities_table;
use unanimitylibrary::database::schema::roles::dsl::roles as roles_table;
use unanimitylibrary::database::schema::roles_capabilities::dsl::roles_capabilities as roles_capabilities_table;
use unanimitylibrary::database::schema::users::dsl::users as users_table;
use unanimitylibrary::database::schema::users_roles::dsl::users_roles as users_roles_table;
use unanimitylibrary::database::Data;
use unanimitylibrary::lib::seeds;

use diesel::query_dsl::RunQueryDsl;

/// Truncate all the tables
pub fn clean() {
    // get connection
    let conn = database_connection();

    // truncate all tables
    diesel::delete(users_roles_table).execute(&conn).unwrap();
    diesel::delete(users_table).execute(&conn).unwrap();
    diesel::delete(addresses_table).execute(&conn).unwrap();
    diesel::delete(roles_capabilities_table)
        .execute(&conn)
        .unwrap();
    diesel::delete(capabilities_table).execute(&conn).unwrap();
    diesel::delete(roles_table).execute(&conn).unwrap();

    // assert empty database
    assert_eq!(users_table.load::<User>(&conn).unwrap().len(), 0);
    assert_eq!(addresses_table.load::<Address>(&conn).unwrap().len(), 0);
    assert_eq!(roles_table.load::<Role>(&conn).unwrap().len(), 0);
    assert_eq!(
        roles_capabilities_table
            .load::<RelRoleCapability>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        capabilities_table.load::<Capability>(&conn).unwrap().len(),
        0
    );
    // assert_eq!(users_roles.load::<>(&conn).unwrap().len(), 0);
}

/// Fill the database with some data that is needed for the application to run
/// correctly.
pub fn seed() {
    let conn = database_connection();

    seeds::roles::seed_roles_and_capabilities(&conn);
}

/// Return a MysqlConnection
/// Since we use a different database for the test environment, this function
/// MUST be used while developping tests.
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

/// Generate a database url from the `.env` file.
/// This database will be used to perform the tests
///
/// Warning : the database will be reset before each test so do
/// not use your regular database.
///
/// The needed informations are
///
/// - TEST_DB_HOST
/// - TEST_DB_PORT
/// - TEST_DB_USER
/// - TEST_DB_PASSWORD
/// - TEST_DB_DATABASE
pub fn database_url() -> String {
    dotenv::dotenv().ok();

    // DB settings
    let db_adapter = "mysql";
    let db_host = env_setting("TEST_DB_HOST");
    let db_port = env_setting("TEST_DB_PORT");
    let db_user = env_setting("TEST_DB_USER");
    let db_password = env_setting("TEST_DB_PASSWORD");
    let db_database = env_setting("TEST_DB_DATABASE");

    // DB url
    format!(
        "{}://{}:{}@{}:{}/{}",
        db_adapter, db_user, db_password, db_host, db_port, db_database
    )
}

/// Set up the Rocket
/// Prepare whatever needs to be prepared so the application can be used
/// through a `rocket::Client`
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

/// Generate a new unique user in database.
///
/// The activation of the user can already be managed from here.
/// It returns the user and its password.
pub fn get_user(do_activate: bool) -> (User, String) {
    use diesel::query_dsl::QueryDsl;
    use unanimitylibrary::database::schema::users::dsl::*;

    let conn = database_connection();

    let last_id = users_table
        .select(id)
        .order(id.desc())
        .first::<u32>(&conn)
        .ok()
        .unwrap_or(0u32)
        + 1;

    let u = UserMinima {
        email: format!("firstname.lastname.{}@student.unamur.be", &last_id),
        password: format!("password_{}", &last_id),
        firstname: format!("Firstname{}", &last_id),
        lastname: format!("Lastname{}", &last_id),
        address: None,
        phone: None,
    };

    let user = match User::insert_minima(&conn, &u) {
        Data::Inserted(u) => u,
        Data::Existing(u) => u,
    };

    if do_activate {
        user.activate(&conn);
    }

    (User::by_email(&conn, &u.email).unwrap(), u.password)
}
