//! # Auth controller
//!
//! Every routes concerning the authentication process are grouped here.

use crate::database::models::prelude::{Address, User};
use crate::database::{DBConnection, Data};
use crate::http::responders::api::ApiResponse;

use crate::auth::forms::{ActivationData, LoginData, RegisterData};
use crate::auth::Auth;
use crate::conf::State;
use crate::lib::mail;

use rocket;
use rocket::http::Status;
use rocket_contrib::json::Json;
use std::convert::identity;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(register, login, activate)
}

/// Create a new user on the platform
///
/// This user MUST have an unique email address.
/// Another validation is performed on the domain of the email address, which
/// MUST be from "unamur.be"
///
/// If the registration succeed, a new user is created in database but this
/// user cannot login right now on the application : he must first activate
/// his account with the link that is sent to his registration email address.
///
/// Right now the activation link is only printed in STDIN (delete this when
/// todo is done).
#[post("/api/v1/auth/register", format = "json", data = "<data>")]
pub fn register(conn: DBConnection, data: Json<RegisterData>) -> ApiResponse {
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

    match User::insert_minima(&conn, &new_user) {
        Data::Existing(_) => ApiResponse::error(Status::Conflict, "Account already exist"),
        Data::Inserted(user) => {
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
        }
        _ => panic!("unreachable code reched"),
    }
}

/// Login as a user on the application
///
/// The application need valid email and password in order to successfully
/// login a user.
/// Once it's done, the client will receive informations about the user that
/// just logged in.
///
/// We choosed to manage the client state with some Json Web Token.
#[post("/api/v1/auth/login", format = "json", data = "<data>")]
pub fn login(conn: DBConnection, state: State, data: Json<LoginData>) -> ApiResponse {
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

/// Activate a user
///
/// The calls to the activation are generated by the application and the client should
/// never be able to create a valid activation link on his own.
///
/// A call to this web service completes the registration process and allows a user to
/// log in the application.
#[post("/api/v1/auth/activate", format = "json", data = "<data>")]
pub fn activate(conn: DBConnection, data: Json<ActivationData>) -> ApiResponse {
    let ActivationData { token, id } = data.into_inner();
    if let Some(user) = User::by_id(&conn, &id) {
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
