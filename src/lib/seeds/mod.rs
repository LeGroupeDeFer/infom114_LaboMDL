use crate::database::tables::*;
use diesel::prelude::*;
use diesel::MysqlConnection;

pub mod posts;
pub mod roles;
pub mod tags;
pub mod users;

pub fn clean_all_table(conn: &MysqlConnection) {
    // truncate all tables
    diesel::delete(roles_capabilities_table)
        .execute(conn)
        .unwrap();
    diesel::delete(users_poll_answers_table)
        .execute(conn)
        .unwrap();
    diesel::delete(poll_answers_table).execute(conn).unwrap();
    diesel::delete(capabilities_table).execute(conn).unwrap();
    diesel::delete(users_roles_table).execute(conn).unwrap();
    diesel::delete(posts_tags_table).execute(conn).unwrap();
    diesel::delete(votes_comments_table).execute(conn).unwrap();
    diesel::delete(votes_posts_table).execute(conn).unwrap();
    diesel::delete(tags_subscription_table)
        .execute(conn)
        .unwrap();
    diesel::delete(roles_table).execute(conn).unwrap();
    diesel::delete(tags_table).execute(conn).unwrap();
    diesel::delete(capabilities_table).execute(conn).unwrap();
    diesel::delete(posts_reports_table).execute(conn).unwrap();
    diesel::delete(comments_table).execute(conn).unwrap();
    diesel::delete(posts_table).execute(conn).unwrap();
    diesel::delete(users_table).execute(conn).unwrap();
    diesel::delete(tokens_table).execute(conn).unwrap();
    diesel::delete(addresses_table).execute(conn).unwrap();
}
