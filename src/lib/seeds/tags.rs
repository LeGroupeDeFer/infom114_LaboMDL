use crate::database::models::tags::tag;
use diesel::MysqlConnection;

pub fn seed_tags(conn: &MysqlConnection) {
    let labels = vec!["Info", "Eco", "Droit", "Pharma"];

    for label in labels {
        tag::Tag::insert(
            &conn,
            &tag::TagMinima {
                label: label.to_string(),
            },
        );
    }
}
