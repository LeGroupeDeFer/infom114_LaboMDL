use crate::database::models::Entity;
use crate::database::models::tag::{Tag, TagMinima};
use diesel::MysqlConnection;

pub fn seed_tags(conn: &MysqlConnection) {
    let labels = vec!["info", "eco", "droit", "pharma"];

    for label in labels {
        Tag::insert(
            &conn,
            &TagMinima {
                label: label.to_string(),
            },
        );
    }
}
