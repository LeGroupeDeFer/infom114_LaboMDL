use crate::database::MyDbConn;
use crate::http::responder::ApiResponse;
use crate::models::quick_response::Info;
use crate::models::user::User;
use crate::schema;

use super::forms::{self, LoginCredentials, RegisterCredentials};
use super::guards::Auth;

use rocket::http::{Cookies, Status};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::json::{Json, JsonError};

use diesel::dsl::count;
use diesel::prelude::*;

pub fn collect() -> Vec<rocket::Route> {
    routes!(post_register, post_register_v1, check_email, check_email_v1)
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["register", "login"]
}

#[post("/api/v1/register", data = "<user_info>")]
fn post_register_v1(
    user_info: Result<Json<RegisterCredentials>, JsonError>,
    conn: MyDbConn,
) -> ApiResponse {
    match user_info {
        Ok(infos) => {
            // TODO : hash password before giving `infos` to diesel
            let _rows_inserted = diesel::insert_into(schema::users::dsl::users)
                .values(&*infos)
                .execute(&*conn);
            ApiResponse::new(Status::Ok, json!(Info::new(true, None)))
        }
        Err(error) => match error {
            JsonError::Io(_) => ApiResponse::bad_request(),
            JsonError::Parse(_, e) => {
                ApiResponse::error(Status::UnprocessableEntity, &e.to_string())
            }
        },
    }
}

#[post("/api/v1/register/check_email", data = "<email_address>")]
fn check_email_v1(
    email_address: Result<Json<forms::Email>, JsonError>,
    conn: MyDbConn,
) -> ApiResponse {
    use schema::users::dsl::users;
    use schema::users::*;
    match email_address {
        Ok(address) => {
            // get count of rows with email corresponding to email
            match users
                .filter(email.eq(&address.email))
                .select(count(id))
                .first::<i64>(&*conn)
            {
                Ok(nbr_rows) => {
                    if nbr_rows == 0 {
                        ApiResponse::simple_success(Status::Ok)
                    } else {
                        ApiResponse::error(
                            Status::Conflict,
                            "This email is already linked to an account.",
                        )
                    }
                }
                Err(e) => ApiResponse::db_error(e),
            }
        }
        Err(error) => ApiResponse::manage_json_error(error),
    }
}

#[post("/api/v1/login", data = "<credentials>")]
fn post_login_v1(
    credentials: Option<Form<LoginCredentials>>,
    conn: MyDbConn,
    mut cookies: Cookies,
) -> Flash<Redirect> {
    match credentials {
        None => Flash::error(Redirect::to("/login"), "Could not retreive credentials"),
        Some(info) => {
            let users = schema::users::dsl::users
                .filter(schema::users::columns::email.eq(&info.email))
                .limit(1)
                .load::<User>(&*conn)
                .expect("user is not in db");

            if users.len() < 1 {
                return Flash::error(Redirect::to("/login"), "User not in DB");
            }

            let user = &users[0];
            if user.password == info.password {
                Auth::login(&mut cookies, &user);

                Flash::success(Redirect::to("/hidden"), "Login successfull")
            } else {
                Flash::error(Redirect::to("/login"), "Wrong password")
            }
        }
    }
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    Auth::logout(&mut cookies);
    return Flash::success(Redirect::to("/login"), "Successfully logout");
}

/* -------------------- Bindings to /api/v<x>/ ----------------------------- */

#[post("/api/register", data = "<user_info>")]
fn post_register(
    user_info: Result<Json<RegisterCredentials>, JsonError>,
    conn: MyDbConn,
) -> ApiResponse {
    post_register_v1(user_info, conn)
}

#[post("/api/login", data = "<credentials>")]
fn post_login(
    credentials: Option<Form<LoginCredentials>>,
    conn: MyDbConn,
    cookies: Cookies,
) -> Flash<Redirect> {
    post_login_v1(credentials, conn, cookies)
}

#[post("/api/register/check_email", data = "<email>")]
fn check_email(email: Result<Json<forms::Email>, JsonError>, conn: MyDbConn) -> ApiResponse {
    check_email_v1(email, conn)
}

/* --------------------------- Tests --------------------------------------- */

// TODO : rewrite tests !
// #[cfg(test)]
// mod test {
//
//     #[test]
//     fn test_collect() {
//         // its ugly af to compare strings output
//         // quicly implemented like this to test code cov
//         assert_eq!(
//             format!(
//                 "{:?}",
//                 routes!(super::post_login, super::logout)
//             ),
//             format!("{:?}", super::collect())
//         );
//     }
// }
