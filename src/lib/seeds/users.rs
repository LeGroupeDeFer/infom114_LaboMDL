use crate::database::models::prelude::{User, UserMinima};
use crate::database::Data;
use diesel::MysqlConnection;

pub fn seed_mock_users(conn: &MysqlConnection) {
    let x = 5;

    // lets create x users
    for i in 1..=x {
        let u = UserMinima {
            email: format!("firstname.lastname.{}@student.unamur.be", i),
            password: format!("password_{}", i),
            firstname: format!("Firstname{}", i),
            lastname: format!("Lastname{}", i),
            address: None,
            phone: None,
        };

        let user = match User::insert_minima(&conn, &u) {
            Data::Inserted(u) => u,
            _ => panic!("The user is supposed to be a new one"),
        };

        user.activate(&conn);
    }
}
