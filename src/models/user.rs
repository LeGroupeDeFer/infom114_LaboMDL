use std::ops::Deref;

use crate::database;
use crate::database::MyDbConn;
use crate::schema;

use diesel::dsl::count;
use diesel::prelude::*;
use diesel::ExpressionMethods;

use regex::Regex;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub street: Option<String>,
    pub number: Option<u32>,
    pub city: Option<String>,
    pub zipcode: Option<u32>,
    pub country: Option<String>,
    pub phone: Option<String>,
}

impl User {
    pub fn cookie(&self) -> String {
        self.id.to_string()
    }

    pub fn from(id: &u32) -> Option<Self> {
        let conn = database::connection();

        let mut users = schema::users::dsl::users
            .filter(schema::users::dsl::id.eq(&id))
            .limit(1)
            .load::<User>(&conn)
            .expect("user is not in db");

        users.pop()
    }

    pub fn check_if_email_is_available(
        email_address: &str,
        conn: &MyDbConn,
    ) -> Result<bool, diesel::result::Error> {
        use schema::users::dsl::users;
        use schema::users::*;

        // get count of rows with email corresponding to email
        match users
            .filter(email.eq(&email_address))
            .select(count(id))
            .first::<i64>(conn.deref())
        {
            Ok(nbr_rows) => {
                if nbr_rows == 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn check_if_email_is_unamur(email_address: &str) -> bool {
        let re = Regex::new(r"^(.*)@(student\.)?unamur\.be$").unwrap();
        re.is_match(email_address)
    }
}
