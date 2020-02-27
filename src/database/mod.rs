use std::collections::BTreeMap;
use std::env;

use dotenv::dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;

use rocket::config::Value;

use rocket_contrib::databases::r2d2::{Pool, PooledConnection};
use rocket_contrib::databases::{DatabaseConfig, Poolable};

const ENV_DATABASE_URL: &'static str = "DATABASE_URL";

pub fn connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = url();
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn url() -> String {
    dotenv().ok();

    // TODO : return Result<String, String>
    env::var(ENV_DATABASE_URL).expect(&format!("{} must be set", ENV_DATABASE_URL))
}

pub struct MyDbConn(pub PooledConnection<<diesel::MysqlConnection as Poolable>::Manager>);

/// The pool type.
pub struct MyDbConnPool(pub Pool<<MysqlConnection as Poolable>::Manager>);

impl MyDbConn {
    /// Returns a fairing that initializes the associated database
    /// connection pool.
    pub fn fairing() -> impl rocket::fairing::Fairing {
        rocket::fairing::AdHoc::on_attach("MyDbConn Database Pool", |rocket| {
            let tab: BTreeMap<String, Value> = BTreeMap::new();

            let db_config = DatabaseConfig {
                url: &url(),
                pool_size: 10,
                extras: tab,
            };

            let pool = <diesel::MysqlConnection as Poolable>::pool(db_config);

            match pool {
                Ok(p) => Ok(rocket.manage(MyDbConnPool(p))),
                Err(pool_error) => {
                    rocket::logger::error("Failed to initialize pool for 'MyDbConn'");
                    rocket::logger::error(&format!("{:?}", pool_error));
                    Err(rocket)
                }
            }
        })
    }

    /// Retrieves a connection of type `Self` from the `rocket`
    /// instance. Returns `Some` as long as `Self::fairing()` has been
    /// attached and there is at least one connection in the pool.
    pub fn get_one(rocket: &rocket::Rocket) -> Option<Self> {
        rocket
            .state::<MyDbConnPool>()
            .and_then(|pool| pool.0.get().ok())
            .map(MyDbConn)
    }
}

impl std::ops::Deref for MyDbConn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MyDbConn {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for MyDbConn {
    type Error = ();

    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, ()> {
        use rocket::{http::Status, Outcome};
        match request.guard::<rocket::State<MyDbConnPool>>() {
            Outcome::Success(pool) => match pool.0.get() {
                Ok(conn) => Outcome::Success(MyDbConn(conn)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            },
            Outcome::Failure(e) => {
                rocket::logger::error("Problem loading MyDbConn");
                rocket::logger::error(&format!("{:?}", e));
                Outcome::Failure((Status::ServiceUnavailable, ()))
            }
            Outcome::Forward(_) => {
                rocket::logger::error("Forwarding WTF");
                Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}
