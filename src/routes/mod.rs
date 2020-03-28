//! # Routes
//!
//! Group all routes of the application, classified by mod respecting the
//! route's path.
//!
//! Some of the routes are just hollow routes that just returns the basic
//! application layout.
//!
//! This is needed since the front is managed by React.

mod api;
use crate::lib::extend_routes;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

pub fn collect() -> Vec<rocket::Route> {
    [
        &routes!(index, dynamic_routing, files, activate, recover)[..],
        &extend_routes("/api", api::collect())[..],
    ]
    .concat()
}

const ALLOWED_ROUTES: [&str; 9] = [
    "profile",
    "notifications",
    "settings",
    "about",
    "login",
    "logout",
    "register",
    "recovery",
    "activate",
];

// /api/<...subpath> => /api/v<version>/<...subpath>

/* --------------------------------- Routes -------------------------------- */

/// Serve application entrypoint
///
/// The React app takes the lead to manage the front-end
#[get("/")]
pub fn index() -> Template {
    Template::render("layout", &())
}

/// Allow some route to return a 200 ok with the react layout template
///
/// Since the client navigation is managed by react, when a client forces the
/// refresh of a page, the backend returns a HTTP 404.
/// This `dynamic_routing` is design to prevent that from happening
#[get("/<route>", rank = 2)]
pub fn dynamic_routing(route: String) -> Option<Template> {
    if ALLOWED_ROUTES.contains(&&route[..]) {
        Some(Template::render("layout", &()))
    } else {
        None
    }
}

/// Hollow route to be accessed by activation link
#[get("/activate/<_id>/<_token>", rank = 1)]
pub fn activate(_id: u32, _token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

/// Hollow route to be accessed by recovery link
#[get("/recover/<_id>/<_token>", rank = 1)]
pub fn recover(_id: u32, _token: String) -> Option<Template> {
    Some(Template::render("layout", &()))
}

/// Serve static files
///
/// Every `js`, `css` or image file found in the `/static/` folder is served
/// with this route.
#[get("/<file..>", rank = 3)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// use crate::database::models::roles::{
//     capability::Capability,
//     forms::RoleData,
//     role::{Role, RoleMinima},
//     role_capability::RelRoleCapability,
// };
// use crate::database::DBConnection;
// use crate::http::responders::api::ApiResponse;
// use rocket::http::Status;
// use rocket_contrib::json::Json;

// #[put("/api/role/<role_id>", format = "json", data = "<data>", rank = 1)]
// pub fn update(conn: DBConnection, role_id: u32, data: Json<RoleData>) -> ApiResponse {
//     let opt_role = Role::from_id(&conn, &role_id);

//     // assert that the role_id given exist
//     if opt_role.is_none() {
//         return ApiResponse::error(
//             Status::UnprocessableEntity,
//             "The targeted role does not exist",
//         );
//     }
//     let role = opt_role.unwrap();

//     // assert that the new name is not already used
//     let role_data = data.into_inner();
//     if Role::from_name(&conn, &role_data.name).is_some() {
//         return ApiResponse::error(Status::Conflict, "A role with this name already exist");
//     }

//     role.update(
//         &conn,
//         &RoleMinima {
//             name: role_data.name.into(),
//             color: role_data.color.into(),
//         },
//     );

//     // reset capabilities
//     role.clear_capabilities(&conn);

//     // add every given capabilities
//     for capability_data in role_data.capabilities.iter() {
//         if let Some(capability) = Capability::from_name(&conn, &capability_data.name) {
//             RelRoleCapability::add_capability_for_role(&conn, &role, &capability);
//         } else {
//             // TODO : front-end sent an unexisting capability
//         }
//     }

//     ApiResponse::new(Status::Ok, json!({}))
// }
