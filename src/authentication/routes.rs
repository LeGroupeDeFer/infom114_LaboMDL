use crate::database::MyDbConn;
use crate::http::responders::api::ApiResponse;
use crate::models::quick_response::Info;
use crate::models::user::User;
use crate::schema;

use super::forms::{self, LoginCredentials, RegisterCredentials};
use super::guards::Auth;

use diesel::prelude::*;
use rocket::http::{Cookies, Status};
use rocket_contrib::json::{Json, JsonError};

pub fn collect() -> Vec<rocket::Route> {
    routes!(
        post_register,
        post_register_v1,
        check_email,
        check_email_v1,
        post_login,
        post_login_v1,
        post_logout,
        post_logout_v1
    )
}

pub fn allowed_paths() -> Vec<&'static str> {
    vec!["register", "login", "logout"]
}

#[post("/api/v1/register", data = "<user_info>")]
fn post_register_v1(
    user_info: Result<Json<RegisterCredentials>, JsonError>,
    conn: MyDbConn,
    mut cookies: Cookies,
) -> ApiResponse {
    if Auth::is_authenticated(&mut cookies) {
        return ApiResponse::error(Status::UnprocessableEntity, "Already authenticated");
    }
    match user_info {
        Ok(infos) => {
            match User::check_if_email_is_available(infos.email, &conn) {
                Ok(is_available) => {
                    if !is_available {
                        return ApiResponse::error(
                            Status::Conflict,
                            "This email is already linked to an account.",
                        );
                    }
                }
                Err(e) => {
                    return ApiResponse::db_error(e);
                }
            }

            // hash password before giving `infos` to diesel
            let hash_res = bcrypt::hash(&infos.password, bcrypt::DEFAULT_COST);
            match hash_res {
                Ok(hash_passwd) => {
                    let user_info_hashed = Json(RegisterCredentials {
                        password: hash_passwd,
                        ..infos.into_inner()
                    });
                    let _rows_inserted = diesel::insert_into(schema::users::dsl::users)
                        .values(&*user_info_hashed)
                        .execute(&*conn);
                    ApiResponse::new(Status::Ok, json!(Info::new(true, None)))
                }
                Err(error) => ApiResponse::error(Status::InternalServerError, &error.to_string()),
            }
        }
        Err(error) => ApiResponse::json_error(error),
    }
}

#[post("/api/v1/register/check_email", data = "<email_address>")]
fn check_email_v1(
    email_address: Result<Json<forms::Email>, JsonError>,
    conn: MyDbConn,
) -> ApiResponse {
    match email_address {
        Ok(address) => {
            match User::check_if_email_is_available(&address.email, &conn) {
                Ok(is_available) => {
                    if is_available {
                        // if no rows returned, it means that the email is still available for account creation
                        ApiResponse::simple_success(Status::Ok)
                    } else {
                        // else, it's not possible to use it to create another account
                        ApiResponse::error(
                            Status::Conflict,
                            "This email is already linked to an account.",
                        )
                    }
                }
                Err(e) => ApiResponse::db_error(e),
            }
        }
        Err(error) => ApiResponse::json_error(error),
    }
}

#[post("/api/v1/login", data = "<credentials>")]
fn post_login_v1(
    credentials: Result<Json<LoginCredentials>, JsonError>,
    conn: MyDbConn,
    mut cookies: Cookies,
) -> ApiResponse {
    match credentials {
        Ok(info) => {
            match schema::users::dsl::users
                .filter(schema::users::columns::email.eq(&info.email))
                .first::<User>(&*conn)
            {
                Ok(user) => match bcrypt::verify(&*info.password, &user.password) {
                    Ok(result) => {
                        if result {
                            Auth::login(&mut cookies, &user);
                            ApiResponse::new(
                                Status::Ok,
                                json!({
                                   "success": true,
                                   "user": {
                                        "id": user.id,
                                        "firstname": user.firstname,
                                        "lastname": user.lastname,
                                        "street": user.street,
                                        "number": user.number,
                                        "city": user.city,
                                        "zipcode": user.zipcode,
                                        "country": user.country,
                                        "phone": user.phone
                                   }
                                }),
                            )
                        } else {
                            ApiResponse::error(
                                Status::Unauthorized,
                                "Wrong email/password association",
                            )
                        }
                    }

                    Err(error) => {
                        ApiResponse::error(Status::InternalServerError, &error.to_string())
                    }
                },
                Err(e) => ApiResponse::db_error(e),
            }
        }
        Err(e) => ApiResponse::json_error(e),
    }
}

#[get("/api/v1/logout")]
fn post_logout_v1(mut cookies: Cookies) -> ApiResponse {
    Auth::logout(&mut cookies);
    ApiResponse::new(Status::Ok, json!(Info::new(true, None)))
}

/* -------------------- Bindings to /api/v<x>/ ----------------------------- */

#[post("/api/register", data = "<user_info>")]
fn post_register(
    user_info: Result<Json<RegisterCredentials>, JsonError>,
    conn: MyDbConn,
    cookies: Cookies,
) -> ApiResponse {
    post_register_v1(user_info, conn, cookies)
}

#[post("/api/login", data = "<credentials>")]
fn post_login(
    credentials: Result<Json<LoginCredentials>, JsonError>,
    conn: MyDbConn,
    cookies: Cookies,
) -> ApiResponse {
    post_login_v1(credentials, conn, cookies)
}

#[get("/api/logout")]
fn post_logout(cookies: Cookies) -> ApiResponse {
    post_logout_v1(cookies)
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
