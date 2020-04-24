use crate::database::models::prelude::{TagEntity, TagMinima};
use diesel::MysqlConnection;

pub fn seed_tags(conn: &MysqlConnection) {
    let labels = vec!["info", "eco", "droit", "pharma"];

    for label in labels {
        TagEntity::insert(
            &conn,
            &TagMinima {
                label: label.to_string(),
            },
        );
    }
}
