use crate::database;
use crate::schema;

use diesel::prelude::*;
use diesel::ExpressionMethods;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl User {
    pub fn cookie(&self) -> String {
        self.id.to_string()
    }

    pub fn from(id: &i32) -> Option<Self> {
        let conn = database::connection();

        let mut users = schema::users::dsl::users
            .filter(schema::users::dsl::id.eq(&id))
            .limit(1)
            .load::<User>(&conn)
            .expect("user is not in db");

        users.pop()
    }

    pub fn test() -> Self {
        User {
            id: 32,
            username: String::from("Cowboy"),
            password: String::from("passwrd"),
            email: Some(String::from("coucouloucoucoupaloma@yopmail.com")),
            firstname: Some(String::from("john")),
            lastname: Some(String::from("Doe")),
        }
    }
}
