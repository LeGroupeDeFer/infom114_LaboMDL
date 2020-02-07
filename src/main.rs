#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;

// #[macro_use]
static DATABASE_URL: &'static str = "mysql://testuser:testpassword@localhost/test_rocket";
use rocket_contrib::templates::Template;

use models::user::User;

#[database("mariadb_test")]
struct MyDbConn(diesel::MysqlConnection);

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[get("/test?<option>")]
fn optional(option: Option<u32>) -> &'static str {
    match option {
        Some(_n) => "Oh ya un nombre",
        None => "rien à voir ici",
    }
}

#[get("/")]
fn index(conn: MyDbConn) -> String {
    // use schema::users::dsl::*;

    let results = schema::users::dsl::users
        // .filter(true)
        .limit(5)
        .load::<User>(&*conn)
        .expect("error loading post");

    format!("{:?}", results)
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
            parent: "layout",
        },
    )
}

/// Serve static files
// #[get("/<file..>")]
// fn files(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).ok()
// }

fn main() {
    rocket::ignite()
        .mount("/", routes![index, optional, hello])
        .attach(Template::fairing())
        .attach(MyDbConn::fairing())
        .launch();
}
