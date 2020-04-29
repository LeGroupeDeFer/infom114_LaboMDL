use rocket::http::{ContentType, Status};

use super::super::init;

use unanimitylibrary::database::models::prelude::*;
const TAG_ROUTE: &'static str = "/api/v1/tag";

/************************************** TEST ***************************************/

#[test]
fn add_new_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();

    let tag = "test";

    // login
    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // load all tags and assert there is none
    assert_eq!(TagEntity::all(&conn).unwrap().len(), 0);

    // create a tag
    let req = client
        .post(format!("{}/{}", TAG_ROUTE, &tag))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check there is only one tag in db, and this tag is the one we just added
    assert_eq!(TagEntity::all(&conn).unwrap().len(), 1);
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());
}

#[test]
fn insert_already_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let tag = "info";

    // login
    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    // assert the seeder did its job well
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());

    let req = client
        .post(format!("{}/{}", TAG_ROUTE, &tag))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is Conflict
    assert_eq!(response.status(), Status::Conflict);

    // assert the tag is still there
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());
}

#[test]
fn insert_tag_without_login() {
    let client = init::clean_client();
    let conn = init::database_connection();

    // load all tags and assert there is none
    assert_eq!(TagEntity::all(&conn).unwrap().len(), 0);

    // create a tag
    let req = client.post(format!("{}/test", TAG_ROUTE));
    let response = req.dispatch();

    //check the answer is forbidden
    assert_eq!(response.status(), Status::Forbidden);

    // assert there is still no tag
    assert_eq!(TagEntity::all(&conn).unwrap().len(), 0);
}

#[test]
fn delete_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let tag = "droit";
    // check that the seeder did its job well
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let req = client
        .delete(format!("{}/{}", TAG_ROUTE, &tag))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    //check the tag does not exist in the db anymore
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_none());
}

#[test]
fn delete_non_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();
    let tag = "nonexisting";

    // check the tag do not already exist
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_none());

    // login as admin
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let req = client
        .delete(format!("{}/{}", TAG_ROUTE, &tag))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is UnprocessableEntity
    assert_eq!(response.status(), Status::BadRequest);

    //check it is still not present in the db
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_none());
}

#[test]
fn delete_tag_without_capability() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let tag = "droit";

    // check that the seeder did its job well
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());

    // login
    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    let req = client
        .delete(format!("{}/{}", TAG_ROUTE, &tag))
        .header(auth_token_header);
    let response = req.dispatch();

    //check the answer is Forbidden
    assert_eq!(response.status(), Status::Forbidden);

    //check the tag was not removed
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_some());
}

#[test]
fn update_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let existing_tag_label = "info";
    let tag = "newinfo";
    let new_tag_json = format!("{{ \"label\": \"{}\" }}", &tag);

    // panics if "existing tag label" does not exist
    let existing_tag = TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .unwrap();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let req = client
        .put(format!("{}/{}", TAG_ROUTE, existing_tag_label))
        .header(auth_token_header)
        .header(ContentType::JSON)
        .body(new_tag_json);
    let response = req.dispatch();

    //check is the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    // check the old tag do not exist anymore
    assert!(TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .is_none());

    // panics if the new label is not found
    let new_tag = TagEntity::by_label(&conn, &tag).unwrap().unwrap();

    // check the id of old and new tag are the same
    assert_eq!(existing_tag.id, new_tag.id);
}

#[test]
fn update_non_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let existing_tag_label = "nonexistingtag";
    let tag = "newinfo";
    let new_tag_json = format!("{{ \"label\": \"{}\" }}", &tag);

    // assert the "existing_tag_label" does not exist
    assert!(TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .is_none());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let req = client
        .put(format!("{}/{}", TAG_ROUTE, existing_tag_label))
        .header(auth_token_header)
        .header(ContentType::JSON)
        .body(new_tag_json);
    let response = req.dispatch();

    //check is the answer is UnprocessableEntity
    assert_eq!(response.status(), Status::BadRequest);

    // check the `existing_tag_label` still do not exist
    assert!(TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .is_none());

    // check the new label do not exist
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_none());
}

#[test]
fn update_tag_with_already_existing_label() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let existing_tag_label = "info";
    let another_tag_label_existing = "pharma";
    let another_tag_json = format!("{{ \"label\": \"{}\" }}", &another_tag_label_existing);

    // panics if "existing tag label" or the "new tag" does not exist
    let existing_tag = TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .unwrap();
    let another_tag = TagEntity::by_label(&conn, &another_tag_label_existing)
        .unwrap()
        .unwrap();

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let req = client
        .put(format!("{}/{}", TAG_ROUTE, existing_tag_label))
        .header(auth_token_header)
        .header(ContentType::JSON)
        .body(another_tag_json);
    let response = req.dispatch();

    //check is the answer is Conflict
    assert_eq!(response.status(), Status::Conflict);

    // check the both tags still exists and are the same
    assert_eq!(
        TagEntity::by_label(&conn, &existing_tag_label)
            .unwrap()
            .unwrap()
            .id,
        existing_tag.id
    );
    assert_eq!(
        TagEntity::by_label(&conn, &another_tag_label_existing)
            .unwrap()
            .unwrap()
            .id,
        another_tag.id
    );
}

#[test]
fn update_tag_without_capability() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let existing_tag_label = "info";
    let tag = "newinfo";
    let new_tag_json = format!("{{ \"label\": \"{}\" }}", &tag);

    // panics if "existing tag label" does not exist
    let existing_tag = TagEntity::by_label(&conn, &existing_tag_label)
        .unwrap()
        .unwrap();

    // login
    let (user, password) = init::get_user(true);
    let auth_token_header = init::login(&user.email, &password);

    let req = client
        .put(format!("{}/{}", TAG_ROUTE, existing_tag_label))
        .header(auth_token_header)
        .header(ContentType::JSON)
        .body(new_tag_json);
    let response = req.dispatch();

    //check is the answer is Forbidden
    assert_eq!(response.status(), Status::Forbidden);

    // check the old tag still exist
    assert_eq!(
        TagEntity::by_label(&conn, &existing_tag_label)
            .unwrap()
            .unwrap()
            .id,
        existing_tag.id
    );

    // check the new tag do not exist
    assert!(TagEntity::by_label(&conn, &tag).unwrap().is_none());
}

#[test]
fn update_tag_with_malformed_json() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let tag_label = "info";

    // assert our label already exist
    assert!(TagEntity::by_label(&conn, &tag_label).unwrap().is_some());

    // login
    let auth_token_header = init::login("admin@unamur.be", "admin");

    let data = "{ \"Malformed\": \"Json\" }";

    let req = client
        .put(format!("{}/{}", TAG_ROUTE, tag_label))
        .body(data)
        .header(ContentType::JSON)
        .header(auth_token_header);
    let response = req.dispatch();

    //check is the answer is Conflict
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // assert the tag is still in database
    assert!(TagEntity::by_label(&conn, &tag_label).unwrap().is_some());
}
