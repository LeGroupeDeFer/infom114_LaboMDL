use crate::database::models::address::Address;
use crate::database::models::user::User;
use crate::database::Connection;
use crate::http::responders::api::ApiResponse;

use crate::auth::Auth;
use crate::authentication::forms::{ActivationData, LoginData, RegisterData};
use crate::conf::State;
use crate::lib::mail;

use rocket;
use rocket::http::Status;
use rocket_contrib::json::Json;
use std::convert::identity;

pub fn collect() -> Vec<rocket::Route> {
    routes!(register, login, activate)
}

#[post("/register", format = "json", data = "<data>")]
fn register(conn: Connection, data: Json<RegisterData>) -> ApiResponse {
    let registration = data.into_inner();
    if !User::is_unamur_email(&registration.email) {
        return ApiResponse::error(
            Status::Unauthorized,
            "Only UNamur staff/students may register.",
        );
    }

    let mut address_id = None;
    if let Some(address) = &registration.address {
        address_id =
            Some(Address::insert_minima(&conn, &address).either(identity, identity)).map(|a| a.id);
    }

    let mut new_user = registration.user();
    new_user.address = address_id;

    User::insert_minima(&conn, &new_user)
        .map_left(|_| ApiResponse::error(Status::Conflict, "Account already exist"))
        .map_right(|user| {
            // TODO, put this email in a dedicated function
            mail::send(
                &user.email,
                "info@unanimity.be",
                &format!(
                    "unanimity.be/activate/{}/{}",
                    &user.id,
                    &user.token.unwrap_or(String::new())
                ),
                vec![],
            );
            ApiResponse::new(Status::Ok, json!({}))
        })
        .either(identity, identity)
}

#[post("/login", format = "json", data = "<data>")]
fn login(conn: Connection, state: State, data: Json<LoginData>) -> ApiResponse {
    let info = data.into_inner();
    let authentication = Auth::login(&conn, &info.email, &info.password);

    if let Some((auth, user)) = authentication {
        if !user.active {
            ApiResponse::new(
                Status::Forbidden,
                json!({ "reason": "This account needs activation" }),
            )
        } else {
            ApiResponse::new(
                Status::Ok,
                json!({
                    "token": auth.token(&state.jwt_secret),
                    "user": user.data()
                }),
            )
        }
    } else {
        ApiResponse::error(
            Status::Unauthorized,
            "The email and password you entered did not match our records. Please double-check and try again."
        )
    }
}

// TODO - Might find a better place with an api/account/activate route
#[post("/activate", format = "json", data = "<data>")]
fn activate(conn: Connection, data: Json<ActivationData>) -> ApiResponse {
    let ActivationData { token, id } = data.into_inner();
    if let Some(user) = User::from(&conn, &id) {
        let activation = user.clone(); // FIXME - Remove clone
        if Some(true) == user.token.map(|account_token| account_token == token) && !user.active {
            activation.activate(&conn);
            return ApiResponse::new(Status::Ok, json!({}));
        }
    }

    ApiResponse::new(
        Status::Forbidden,
        json!({ "reason": "Incorrect activation scheme" }),
    )
}

/*
// TODO - Might find a better place with an api/account/recover route
#[post("/recovery", format = "json", data = "<data>")]
fn recovery(conn: Connection, data: Json<ActivationData>) -> ApiResponse {
    let ActivationData { token, id } = data.into_inner();
    if let Some(user) = User::from(&conn, &id) {
        let activation = user.clone(); // FIXME - Remove clone
        if Some(true) == user.token.map(|account_token| account_token == token) {
            activation.recover(&conn);
            return ApiResponse::new(Status::Ok, json!({}));
        }
    }

    ApiResponse::new(Status::Forbidden, json!({ "reason": "Incorrect token" }))
}
*/
/*
#[post("/check_email", data = "<email_address>")]
fn check_email(
    email_address: Result<Json<forms::Email>, JsonError>,
    conn: Connection,
) -> ApiResponse {
    match email_address {
        Ok(address) => {
            if User::is_available_email(&conn, &address.email) {
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
*/

/*
#[get("/logout")]
fn post_logout(mut cookies: Cookies) -> ApiResponse {
    Auth::logout(&mut cookies);
    ApiResponse::new(Status::Ok, json!(Info::new(true, None)))
}
*/
