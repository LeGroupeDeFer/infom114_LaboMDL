use super::super::init;

#[test]
fn user_has_capability() {
    init::clean();
    init::seed();
    let conn = init::database_connection();
    let admin = init::get_admin();
    let capability = "post:delete";

    assert!(admin.has_capability(&conn, &capability));
}

#[test]
fn user_has_not_capability() {
    init::clean();
    init::seed();
    let conn = init::database_connection();
    let (user, _) = init::get_user(true);
    let capability = "post:delete";

    assert!(!user.has_capability(&conn, &capability));
}
