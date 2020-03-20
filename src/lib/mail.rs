// This function is a mock of a true mailing function while a better API is developped
pub fn send(from: &str, to: &str, message: &str, cc: Vec<&str>) {
    let others = cc
        .iter()
        .fold(String::new(), |acc, email| format!("{} {}", &acc, email));
    println!(" -------------------- MAIL -------------------- ");
    println!("FROM: {}", from);
    println!("TO: {} (CC: {})", to, others);
    println!("{}", &message);
    println!(" ------------------ END MAIL ------------------ ");
}
