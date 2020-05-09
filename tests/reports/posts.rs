use super::super::init;
use rocket::http::Status;
use unanimitylibrary::database::models::prelude::PostReport;

#[test]
fn get_post_report() {
    let client = init::clean_client();
    init::seed();

    let mut response = client
        .get("/api/v1/report/activity")
        .header(init::login_admin())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let data: Vec<PostReport> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    for post_report in data {
        match post_report.month.as_ref() {
            "janvier" => {
                assert_eq!(post_report.interaction, 0);
                assert_eq!(post_report.new, 0);
            }
            _ => {}
        }
    }
}
