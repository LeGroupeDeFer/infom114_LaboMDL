use crate::database;
use crate::schema;

use diesel::prelude::*;
use diesel::ExpressionMethods;

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
}
