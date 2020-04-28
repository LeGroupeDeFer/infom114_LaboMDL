use crate::lib::consequence::*;
use crate::database::models::prelude::*;


// This function is a mock of a true mailing function while a better API is developped
pub fn send(to: &UserEntity, message: &str, cc: &Vec<&UserEntity>) -> Consequence<bool>{
    let others = cc
        .iter()
        .fold(String::new(), |acc, user| format!("{} {}", &acc, &user.email));

    println!(" -------------------- MAIL -------------------- ");
    println!("FROM: info@unanimity.be");
    println!("TO: {} (CC: {})", &to.email, others);
    println!("{}", &message);
    println!(" ------------------ END MAIL ------------------ ");

    Ok(true) // bool is dummy, this method will almost certainly have errors to handle so we set
    // a dummy while it's not implemented
}
