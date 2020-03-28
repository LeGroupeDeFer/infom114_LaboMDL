use crate::database::models::address::Address;
use crate::database::models::user::User;
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

pub fn collect() -> Vec<rocket::Route> {
    routes!(register, login, activate)
}

#[post("/register", format = "json", data = "<data>")]
fn register(conn: DBConnection, data: Json<RegisterData>) -> ApiResponse {
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
    }
}

#[post("/login", format = "json", data = "<data>")]
fn login(conn: DBConnection, state: State, data: Json<LoginData>) -> ApiResponse {
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
fn activate(conn: DBConnection, data: Json<ActivationData>) -> ApiResponse {
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
