use crate::database::models::tag::{TagEntity, TagMinima};
use crate::database::models::Entity;
use diesel::MysqlConnection;

pub fn seed_tags(conn: &MysqlConnection) {
    let labels = vec!["info", "eco", "droit", "pharma", "hollow"];

    seed(conn, &labels);
}

fn seed(conn: &MysqlConnection, labels: &[&str]) {
    for label in labels {
        TagEntity::insert(
            &conn,
            &TagMinima {
                label: label.to_string(),
            },
        )
        .unwrap();
    }
}

pub fn seed_prod_tags(conn: &MysqlConnection) {
    let labels = vec![
        "info",
        "med",
        "eco",
        "philo",
        "vété",
        "droit",
        "sciences",
        "arsenal",
        "étudiant",
        "chimie",
        "biologie",
        "géologie",
        "géographie",
        "machinelearning",
        "cours",
        "rectorat",
        "secrétariat",
        "activité",
        "conférence",
        "mémoire",
        "tfe",
        "stage",
        "coronavirus",
        "kap",
        "cercle",
        "campus",
        "sport",
        "",
    ];

    seed(conn, &labels);
}
