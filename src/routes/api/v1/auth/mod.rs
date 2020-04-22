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
  routes!(register, login, logout, activate, restore, recover, refresh)
}


#[post("/register", format = "json", data = "<data>")]
fn register(conn: DBConnection, data: Json<RegisterData>) -> ApiResult<()> {
    let registration = data.into_inner();

    // Get the necessary data
    let address = registration.clone().address.map(|address| {
        Address::insert_either(&conn, &address).map(|a| a.id)
    }).transpose()?;

    let activation_token = Token::create_default(&conn)?;

    let mut minima = UserMinima::from(&registration.clone());
    minima.address = address;
    minima.activation_token = Some(activation_token.id);

    // Create the user
    let user = User::insert_new(&conn, &minima)?;

    // Send the activation link to the user
    mail::send(
        &user,
        &format!("unanimity.be/activate/{}/{}", &user.id, &activation_token.hash),
        &vec![]
    )?;

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

#[post("/logout", format = "json", data = "<data>")]
fn logout(conn: DBConnection, data: Json<LogoutData>) -> ApiResult<()> {
    let LogoutData { email, refresh_token } = data.into_inner();
    Auth::logout(&conn, &email, &refresh_token)?;

    OK()
}

#[post("/activate", format = "json", data = "<data>")]
fn activate(conn: DBConnection, state: State, data: Json<ActivationData>) -> ApiResult<()> {
    let ActivationData { id, token } = data.into_inner();

    let mut user = User::of(&conn, &id)??;
    if user.active {
        return Err(AuthError::AlreadyActivated)?;
    }

    let mut activation_token = user.activation_token(&conn)?.map_or_else(
        || Err(AuthError::InvalidToken),
        |v| Ok(v)
    )?;

    activation_token.vouch(&conn, &token)?;

    let recovery_token = Token::create_default(&conn)?;
    let refresh_token = Token::create(
        &conn,
        Some(&state.access_lifetime),
        Some(&-1)
    )?;

    user.recovery_token = Some(recovery_token.id);
    user.refresh_token = Some(refresh_token.id);
    user.active = true;
    user.update(&conn)?;

    OK()
}

#[post("/restore", format = "json", data = "<data>")]
fn restore(conn: DBConnection, data: Json<RestoreData>) -> ApiResult<()> {
    let email = data.into_inner().email;

    let mut user = User::by_email(&conn, &email)??;
    let mut recovery_token = user.recovery_token(&conn)?.map_or_else(
        || Err(AuthError::InvalidToken),
        |v| Ok(v)
    )?;
    recovery_token.renew(&conn, Some(&(3600)), Some(&1))?;
    user.recovery_token = Some((&recovery_token).id);
    user.update(&conn)?;

    mail::send(
        &user, &format!("unanimity.be/recover/{}/{}", &user.id, &recovery_token.hash), &vec![]
    )?;

    OK()
}

#[post("/recover", format = "json", data = "<data>")]
fn recover(conn: DBConnection, data: Json<RecoveryData>) -> ApiResult<()> {
    let RecoveryData { id, password, token } = data.into_inner();

    let mut user = User::of(&conn, &id)??;
    let mut recovery_token = user.recovery_token(&conn)?.map_or_else(
        || Err(AuthError::InvalidToken),
        |v| Ok(v)
    )?;

    recovery_token.vouch(&conn, &token)?;

    user.set_password(&password)?;
    user.update(&conn)?;

    OK()
}

#[post("/refresh", format = "json", data = "<data>")]
fn refresh(conn: DBConnection, state: State, data: Json<RefreshData>) -> ApiResult<RefreshSuccess> {
    let RefreshData { email, refresh_token } = data.into_inner();
    let (auth, token, user) = Auth::refresh(
        &conn,
        &email,
        &refresh_token,
        &state.access_lifetime,
        &state.refresh_lifetime
    )?;

    Ok(Json(RefreshSuccess {
        access_token: auth.token(&state.jwt_secret)?,
        user: user.data(),
        refresh_token: token.hash
    }))
}
