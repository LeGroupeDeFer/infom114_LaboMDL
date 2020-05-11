use super::super::init;
use rocket::http::Status;
use unanimitylibrary::database::models::prelude::TagReport;

#[test]
fn get_tag_report() {
    let client = init::clean_client();
    init::seed();

    let mut response = client
        .get("/api/v1/report/tags")
        .header(init::login_admin())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let data: Vec<TagReport> = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    let mut tag_even = false;
    let mut tag_odd = false;
    let mut tag_droit = false;
    let mut tag_eco = false;
    let mut tag_info = false;
    let mut tag_pharma = false;
    for tag_report in data {
        match tag_report.tag.as_ref() {
            "droit" => {
                tag_droit = true;
                assert_eq!(tag_report.info, 0);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }
            "eco" => {
                tag_eco = true;
                assert_eq!(tag_report.info, 0);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }
            "info" => {
                tag_info = true;
                assert_eq!(tag_report.info, 0);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }
            "even" => {
                tag_even = true;
                assert_eq!(tag_report.info, 2);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }
            "odd" => {
                tag_odd = true;
                assert_eq!(tag_report.info, 3);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }

            "pharma" => {
                tag_pharma = true;
                assert_eq!(tag_report.info, 0);
                assert_eq!(tag_report.idea, 0);
                assert_eq!(tag_report.poll, 0);
            }
            _ => {}
        }
    }

    assert!(tag_even);
    assert!(tag_odd);
    assert!(tag_droit);
    assert!(tag_eco);
    assert!(tag_info);
    assert!(tag_pharma);
}
