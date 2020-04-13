//! # Update roles
//!
//! Here are grouped the tests that are meant to update role related data.
//!

/************************* REQUIRE *******************************************/

use rocket::http::ContentType;
use rocket::http::Status;

use super::super::init;

const ROLE_ROUTE: &'static str = "/api/role/";

use unanimitylibrary::database::models::roles;
use unanimitylibrary::database::Data;

/**************************** TESTS ******************************************/

#[test]
fn update_everything() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert there is a role with this new name in database
    let role_option = roles::role::Role::by_name(&conn, role_name);
    assert!(role_option.is_some());
    let role = role_option.unwrap();

    assert_eq!(role_name, role.name);
    assert_eq!(role_color, role.color);
    // if it panics, the test cannot pass !
    let role_capa = roles::RoleCapabilities::by_role_name(&conn, &role.name).unwrap();
    assert_eq!(role_capa.capabilities.len(), role_capabilities.len());
    for capability in role_capa.capabilities {
        assert!(role_capabilities.contains(&&capability.name[..]));
    }
}

#[test]
fn update_same_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role exists
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "mynewrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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

    // assert no role with this name already exists (since its the same name)
    assert!(roles::role::Role::by_name(&conn, role_name).is_some());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Ok);

    // assert there is a role with this new name in database
    let role_option = roles::role::Role::by_name(&conn, role_name);
    assert!(role_option.is_some());
    let role = role_option.unwrap();

    assert_eq!(role_name, role.name);
    assert_eq!(role_color, role.color);
    // if it panics, the test cannot pass !
    let role_capa = roles::RoleCapabilities::by_role_name(&conn, &role.name).unwrap();
    assert_eq!(role_capa.capabilities.len(), role_capabilities.len());
    for capability in role_capa.capabilities {
        assert!(role_capabilities.contains(&&capability.name[..]));
    }
}

#[test]
fn update_missing_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/", ROLE_ROUTE))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn update_invalid_role_id() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll find an unexisting role id
    let mut fake_id = 11;
    while let Some(_) = roles::role::Role::by_id(&conn, &fake_id) {
        fake_id += 11;
    }

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, fake_id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_no_color() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_missing_role_name() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_missing_role_capabilities() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";

    // craft body
    let data = format!(
        "{{
        \"name\": \"{}\",
        \"color\": \"{}\"
    }}",
        role_name, role_color
    );

    // assert no role with this name already exists
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn update_without_correct_capability() {
    // init
    let client = init::clean_client();
    init::seed();
    let conn = init::database_connection();

    // first we'll add a role
    let role_minima = roles::role::RoleMinima {
        name: "mynewrole".to_string(),
        color: "#ff0000".to_string(),
    };

    let existing_role = match roles::role::Role::insert_minima(&conn, &role_minima) {
        Data::Inserted(r) => r,
        _ => panic!("should be a new role"),
    };
    // assert the role is correctly added in database
    assert!(roles::role::Role::by_name(&conn, &role_minima.name).is_some());

    // login
    let (user, passwd) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &passwd);

    let role_name = "myupdatedrole";
    let role_color = "#00ffff";
    let role_capabilities = vec!["post:create", "role:manage", "user:manage"];

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
    assert!(roles::role::Role::by_name(&conn, role_name).is_none());

    // request
    let request = client
        .put(format!("{}/{}", ROLE_ROUTE, existing_role.id))
        .header(ContentType::JSON)
        .header(auth_token_header)
        .body(data);
    let response = request.dispatch();

    // validate status
    assert_eq!(response.status(), Status::Forbidden);
}
