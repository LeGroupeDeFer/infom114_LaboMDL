//! # Create roles
//!
//! Here are grouped the tests that are meant to create role related data.
//!

/************************* REQUIRE *******************************************/

use rocket::http::ContentType;
use rocket::http::Status;

use super::super::init;

const ROLE_ROUTE: &'static str = "/api/role/";

use unanimitylibrary::database::models::roles;

/**************************** TESTS ******************************************/

#[test]
fn create_correct_role() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#ff0000";
    let role_capabilities = vec!["post:create", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert there is a role with this new name in database
    let role_option = roles::role::Role::from_name(&conn, role_name);
    assert!(role_option.is_some());
    let role = role_option.unwrap();

    assert_eq!(role_name, role.name);
    assert_eq!(role_color, role.color);
    // if it panics, the test cannot pass !
    let role_capa = roles::RoleCapabilities::from_role_name(&conn, &role.name).unwrap();
    assert_eq!(role_capa.capabilities.len(), role_capabilities.len());
    for capability in role_capa.capabilities {
        assert!(role_capabilities.contains(&&capability.name[..]));
    }
}

#[test]
fn create_role_missing_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#ff0000";
    let role_capabilities = vec!["user:manage", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn create_role_empty_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#ff0000";
    let role_capabilities = vec!["post:create", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn create_role_missing_color() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_capabilities = vec!["post:create", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn create_role_missing_capabilities() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#ff0000";

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\"
    }}",
        role_name, role_color,
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn create_role_unexistant_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#ff0000";
    let role_capabilities = vec![
            "thiscapability:donotexist",
            "post:create",
            "role:manage",
        ];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn create_existing_role() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "admin"; // the admin role is created at the `init::seed()` step
    let role_color = "#ff0000";
    let role_capabilities = vec!["post:create", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert a role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_some());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Conflict);
}

// TODO : create role with a user that do not have the right to do so

#[test]
fn create_correct_role_missing_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();
    let (user, passwd) = init::get_user(true);

    // login
    let auth_token_header = init::login(&user.email, &passwd);

    let role_name = "mynewrole";
    let role_color = "#ff0000";
    let role_capabilities = vec!["post:create", "role:manage"];

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\",
        \"capabilities\": [{}]
    }}",
        role_name,
        role_color,
        role_capabilities
            .iter()
            .map(|cap| format!("{{ \"name\" : \"{}\" }}", cap))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::from_name(&conn, role_name).is_none());

    // request
    let request = client
        .post(ROLE_ROUTE)
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Forbidden);
}
