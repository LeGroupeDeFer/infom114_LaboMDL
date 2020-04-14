use crate::database::models::prelude::*;
use crate::auth::forms::*;

use crate::database::DBConnection;
use crate::http::responders::{ApiResult, OK};
use crate::auth::Auth;
use crate::conf::State;
use crate::lib::mail;

use rocket;
use rocket_contrib::json::Json;

pub fn collect() -> Vec<rocket::Route> {
  routes!(register, login, activate, restore, recovery, refresh)
}

#[post("/register", format = "json", data = "<data>")]
fn register(conn: DBConnection, data: Json<RegisterData>) -> ApiResult<()> {
    let registration = data.into_inner();

    // Get the necessary data
    let address = registration.clone().address.map(|address| {
        Address::insert_either(&conn, &address).map(|a| a.id)
    }).transpose()?;
    let token = Token::create_default(&conn)?;
    let mut minima = UserMinima::from(&registration.clone());
    minima.address = address;
    minima.activation_token = Some(token.id);

    // Create the user
    let user = User::insert_new(&conn, &minima)?;

    // Send the activation link to the user
    mail::send(&user, &format!("unanimity.be/activate/{}/{}", &user.id, &token.hash), &vec![])?;

    OK()
}

#[post("/login", format = "json", data = "<data>")]
fn login(conn: DBConnection, state: State, data: Json<LoginData>) -> ApiResult<LoginSuccess> {
    let info = data.into_inner();
    let (auth, refresh, user) = Auth::login(
        &conn,
        &info.email,
        &info.password,
        &state.access_lifetime,
        &state.refresh_lifetime
    )?;

    Ok(Json(LoginSuccess {
        access_token: auth.token(&state.jwt_secret)?,
        refresh_token: refresh.hash,
        user: user.data()
    }))
}

#[post("/activate", format = "json", data = "<data>")]
fn activate(conn: DBConnection, data: Json<ActivationData>) -> ApiResult<()> {
    let ActivationData { id, token } = data.into_inner();

    let mut user = User::of(&conn, &id)??;
    if user.active {
        return Err(AuthError::AlreadyActivated)?;
    }

    let mut activation_token = user.activation_token(&conn)?.map_or_else(
        || Err(AuthError::InvalidToken),
        |v| Ok(v)
    )?;

    activation_token.verify(&token)?;
    activation_token.consume(&conn)?;

    user.active = true;
    user.update(&conn)?;

    OK()
}

#[post("/restore", format = "json", data = "<data>")]
fn restore(conn: DBConnection, data: Json<RestoreData>) -> ApiResult<()> {
    let email = data.into_inner().email;

    let mut user = User::by_email(&conn, &email)??;
    if user.active {
        return Err(AuthError::AlreadyActivated)?;
    }

    // Send the activation link to the user
    let mut activation_token = user.activation_token(&conn)?.unwrap_or(
        Token::create_default(&conn)?
    );
    activation_token.renew(&conn, Some(1))?;
    user.activation_token = Some((&activation_token).id);
    user.update(&conn)?;

    mail::send(
        &user, &format!("unanimity.be/activate/{}/{}", &user.id, &activation_token.hash), &vec![]
    )?;

    OK()
}

#[post("/recovery", format = "json", data = "<data>")]
fn recovery(conn: DBConnection, data: Json<RecoveryData>) -> ApiResult<()> {
    let RecoveryData { email, password, token } = data.into_inner();

    let user = User::by_email(&conn, &email)??;
    if !user.verify(&password)? { Err(AuthError::InvalidIDs)?; }
    let mut recovery_token = user.recovery_token(&conn)?.unwrap_or(
        Err(AuthError::InvalidToken)?
    );

    recovery_token.verify(&token)?;
    recovery_token.consume(&conn)?;

    OK()
}

#[post("/refresh", format = "json", data = "<data>")]
fn refresh(conn: DBConnection, state: State, data: Json<RefreshData>) -> ApiResult<RefreshSuccess> {
    let RefreshData { email, refresh_token } = data.into_inner();
    let auth = Auth::refresh(&conn, &email, &refresh_token, &state.access_lifetime)?;

    Ok(Json(RefreshSuccess {
        access_token: auth.token(&state.jwt_secret)?
    }))
}
