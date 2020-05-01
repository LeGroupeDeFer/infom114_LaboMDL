//! # Seeder
//!
//! This is a helper used to hydrate the database so the application can be
//! used right after install

use unanimitylibrary::database;
use unanimitylibrary::lib::seeds;

/// Seed database
fn main() {
    let conn = database::connection(&database::url());
    seeds::clean_all_table(&conn);
    seeds::roles::seed_roles_and_capabilities(&conn);
    seeds::tags::seed_tags(&conn);
    seeds::posts::seed_test_posts(&conn);
}
