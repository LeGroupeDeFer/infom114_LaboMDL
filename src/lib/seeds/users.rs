use crate::database::models::prelude::{Entity, Token, Result};
use crate::database::models::user::{User, UserMinima};
use diesel::MysqlConnection;

pub fn seed_mock_users(conn: &MysqlConnection) -> Result<()>{
    let x = 5;

    // lets create x users
    for i in 1..=x {
        let activation_token = Token::create_default(conn)?;
        let recovery_token = Token::create_default(conn)?;
        let refresh_token = Token::create_default(conn)?;

        let u = UserMinima {
            email: format!("firstname.lastname.{}@student.unamur.be", i),
            password: format!("password_{}", i),
            firstname: format!("Firstname{}", i),
            lastname: format!("Lastname{}", i),
            address: None,
            phone: None,
            activation_token: Some(activation_token.id),
            recovery_token: Some(recovery_token.id),
            refresh_token: Some(refresh_token.id)
        };

        let mut user = User::insert_new(&conn, &u).unwrap();

        user.activate(&conn);
    }

    Ok(())
}
