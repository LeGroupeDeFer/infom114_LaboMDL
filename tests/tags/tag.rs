use rocket::http::Status;

use super::super::init;

use unanimitylibrary::database::models::tags::tag::Tag;
use unanimitylibrary::database::schema::tags::dsl::tags as table;
use unanimitylibrary::database::schema::tags;


/************************************** TEST ***************************************/

#[test]
fn add_new_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();

    let req = client.post("/api/tag/test");
    let response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    //load all tags in the db and assert it contains the new tag test
    let db_tags = Tag::all(&conn);
    assert!(db_tags.iter().any(|tag| tag.label=="test"));
    
    //check there is only one tag in the db 
    assert_eq!(db_tags.len(), 1);
}


#[test]
fn insert_already_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();
    let req = client.post("/api/tag/Info");  //it is created by default in the seed
    let response = req.dispatch();

    //check the answer is Conflict
    assert_eq!(response.status(), Status::Conflict);

}

#[test]
fn delete_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();
    let req = client.delete("/api/tag/Droit");  //it is created by default in the seed
    let response = req.dispatch();

    //check the answer is Ok
    assert_eq!(response.status(), Status::Ok);

    //check the tag does not exist in the db anymore 
    let db_tags = Tag::all(&conn);
    assert!( !(db_tags.iter().any(|tag| tag.label=="droit")));
    
}

#[test]
fn delete_non_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();
    let req = client.delete("/api/tag/nonExisting");  //it is created by default in the seed
    let response = req.dispatch();

    //check the answer is UnprocessableEntity
    assert_eq!(response.status(), Status::UnprocessableEntity);

    //check it is not present in the db
    let db_tags = Tag::all(&conn);
    assert!( !(db_tags.iter().any(|tag| tag.label=="nonExisting")));

}
#[test]
fn update_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let new_label = "{
        \"label\": \"NewInfo\"
    }";

    let req = client.put("/api/tag/Info").body(new_label);  //it is created by default in the seed
    let response = req.dispatch();

    //check is the answer is Ok
    assert_eq!(response.status(), Status::Ok)
}

#[test]
fn update_non_existing_tag() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let new_label = "{
        \"label\": \"NewLabel\"
    }";

    let req = client.put("/api/tag/nonExisting").body(new_label);  //it is created by default in the seed
    let response = req.dispatch();

    //check is the answer is UnprocessableEntity
    assert_eq!(response.status(), Status::UnprocessableEntity)
}

#[test]
fn update_tag_with_already_existing_label() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let new_label = "{
        \"label\": \"Info\"
    }";

    let req = client.put("/api/tag/Pharma").body(new_label);  //it is created by default in the seed
    let response = req.dispatch();

    //check is the answer is Conflict
    assert_eq!(response.status(), Status::Conflict)
}

#[test]
fn update_tag_with_malformed_json() {
    let client = init::clean_client();
    let conn = init::database_connection();
    init::seed();

    let new_label = "{
        \"Malformed\": \"Json\"
    }";

    let req = client.put("/api/tag/Info").body(new_label);  //it is created by default in the seed
    let response = req.dispatch();

    //check is the answer is Conflict
    assert_eq!(response.status(), Status::UnprocessableEntity);
}