//! # Test init
//!
//! Initialisations and helpers to ease the development of automated tests.
//!
//! the test database MUST be available
//! the migrations MUST have been applied to the test database

use unanimitylibrary::conf::env_setting;

use unanimitylibrary::database;
use unanimitylibrary::database::models::prelude::*;

use unanimitylibrary::database::tables::*;

use unanimitylibrary::database::Data;
use unanimitylibrary::lib::seeds;

use diesel::query_dsl::RunQueryDsl;

use rocket::http::{ContentType, Header};

/// Truncate all the tables
pub fn clean() {
    // get connection
    let conn = database_connection();

    // truncate all tables
    diesel::delete(users_roles_table).execute(&conn).unwrap();
    diesel::delete(posts_tags_table).execute(&conn).unwrap();
    diesel::delete(roles_capabilities_table)
        .execute(&conn)
        .unwrap();
    diesel::delete(votes_comments_table).execute(&conn).unwrap();
    diesel::delete(votes_posts_table).execute(&conn).unwrap();
    diesel::delete(tags_subscription_table)
        .execute(&conn)
        .unwrap();
    diesel::delete(roles_table).execute(&conn).unwrap();
    diesel::delete(capabilities_table).execute(&conn).unwrap();
    diesel::delete(users_table).execute(&conn).unwrap();
    diesel::delete(addresses_table).execute(&conn).unwrap();
    diesel::delete(tags_table).execute(&conn).unwrap();
    diesel::delete(posts_table).execute(&conn).unwrap();

    // assert empty database
    assert_eq!(
        users_roles_table
            .load::<RelUserRoleEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        posts_tags_table
            .load::<RelPostTagEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        roles_capabilities_table
            .load::<RelRoleCapabilityEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        votes_comments_table
            .load::<RelCommentVoteEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        votes_posts_table
            .load::<RelPostVoteEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        tags_subscription_table
            .load::<RelUserTagEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(roles_table.load::<RoleEntity>(&conn).unwrap().len(), 0);
    assert_eq!(
        capabilities_table
            .load::<CapabilityEntity>(&conn)
            .unwrap()
            .len(),
        0
    );
    assert_eq!(users_table.load::<UserEntity>(&conn).unwrap().len(), 0);
    assert_eq!(
        addresses_table.load::<AddressEntity>(&conn).unwrap().len(),
        0
    );
    assert_eq!(tags_table.load::<TagEntity>(&conn).unwrap().len(), 0);
    assert_eq!(posts_table.load::<PostEntity>(&conn).unwrap().len(), 0);
}

/// Fill the database with some data that is needed for the application to run
/// correctly.
pub fn seed() {
    let conn = database_connection();

    seeds::roles::seed_roles_and_capabilities(&conn);
    seeds::tags::seed_tags(&conn);
}

/// Return a MysqlConnection
/// Since we use a different database for the test environment, this function
/// MUST be used while developing tests.
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
/// The needed information are
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
pub fn get_user(do_activate: bool) -> (UserEntity, String) {
    let conn = database_connection();

    let last_id = UserEntity::get_last_id(&conn) + 1;

    let u = UserMinima {
        email: format!("firstname.lastname.{}@student.unamur.be", &last_id),
        password: format!("password_{}", &last_id),
        firstname: format!("Firstname{}", &last_id),
        lastname: format!("Lastname{}", &last_id),
        address: None,
        phone: None,
    };

    let user = match UserEntity::insert_minima(&conn, &u) {
        Data::Inserted(u) => u,
        _ => panic!("The user is supposed to be a new one"),
    };

    if do_activate {
        user.activate(&conn);
    }

    (UserEntity::by_email(&conn, &u.email).unwrap(), u.password)
}

/// Get the admin that is generated in the seeding process
/// The admin has by default the following characteristics :
///
///     - email : "admin@unamur.be"
///     - password : "admin"
///
/// Of course these attributes MUST be updated ASAP for real world application
/// but for our testing purposes its perfect because we can use it to confirm
/// that some routes are protected by auth & by capabilities
pub fn get_admin() -> UserEntity {
    UserEntity::by_email(&database_connection(), "admin@unamur.be").unwrap()
}

/// Perform the login operation for the given `email` & `password`
///
/// Since it's designed for testing purposes, it will panic if the credentials
/// are wrong.
///
/// This function returns a header that can instantly be used in a
/// `ClientRequest` build.
pub fn login<'a, 'b>(email: &'a str, password: &'a str) -> Header<'b> {
    use serde_json::Value;

    // get the client
    let client = client();
    let login_url = "/api/auth/login";

    // create the json body
    let json_credentials = format!(
        "{{\"email\":\"{}\", \"password\": \"{}\"}}",
        email, password
    );

    // perform the login
    let mut response = client
        .post(login_url)
        .header(ContentType::JSON)
        .body(json_credentials)
        .dispatch();

    // get valuable data (the auth token)
    let content = response.body_string().unwrap();
    let data: Value = serde_json::from_str(&content).unwrap();
    let auth_token = data["token"].to_string();

    Header::new(
        "authorization",
        format!(
            "{}{}",
            unanimitylibrary::guards::auth::TOKEN_PREFIX,
            // ugly hack to have something working
            auth_token.replace("\"", "")
        ),
    )
}
