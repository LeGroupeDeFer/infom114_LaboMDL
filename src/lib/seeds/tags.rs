use crate::database::models::tags::{tag, tag::TagMinima};
use crate::database::models::user;
use crate::database::Data;
use diesel::MysqlConnection;

pub fn seed_tags(conn: &MysqlConnection) {
    // create some minima tags 
    let info_minima = tag::TagMinima {
        label : "Info".to_string(), 
    };
    let eco_minima = tag::TagMinima {
        label : "Eco".to_string(),
    };
    let droit_minima = tag::TagMinima {
        label : "Droit".to_string(),
    };
    let pharma_minima = tag::TagMinima {
        label : "Pharma".to_string(),
    };

    // insert those roles in database
    tag::Tag::insert(&conn, &info_minima);
    tag::Tag::insert(&conn, &eco_minima);
    tag::Tag::insert(&conn, &droit_minima);
    tag::Tag::insert(&conn, &pharma_minima);

}
