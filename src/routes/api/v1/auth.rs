//! # Auth controller
//!
//! Every routes concerning the authentication process are grouped here.
use rocket;
use rocket_contrib::json::Json;

use crate::conf::State;
use crate::database::models::prelude::*;
use crate::database::DBConnection;
use crate::guards::*;
use crate::http::responders::{ApiResult, OK};
use crate::lib::consequence::*;
use crate::lib::mail;

/// Collect every routes that this module needs to share with the application
/// The name `collect` is a project convention
pub fn collect() -> Vec<rocket::Route> {
    routes!(register, login, logout, activate, restore, recover, refresh)
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
/// Right now the activation link is only printed to STDOUT (delete this when
/// todo is done).
#[post("/api/v1/auth/register", format = "json", data = "<data>")]
fn register(conn: DBConnection, data: Json<RegisterData>) -> ApiResult<()> {
    let registration = data.into_inner();

    // Get the necessary data
    let address = registration
        .clone()
        .address
        .map(|address| AddressEntity::insert_either(&*conn, &address).map(|a| a.id))
        .transpose()?;

    let activation_token = TokenEntity::create_default(&*conn)?;

    let mut minima = UserMinima::from(&registration.clone());
    minima.address = address;
    minima.activation_token = Some(activation_token.id);

    // Create the user
    let user = UserEntity::insert_new(&*conn, &minima)?;

    // Send the activation link to the user
    mail::send(
        &user,
        &format!(
            "unanimity.be/activate/{}/{}",
            &user.id, &activation_token.hash
        ),
        &vec![],
    )?;

    OK()
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
fn login(conn: DBConnection, state: State, data: Json<LoginData>) -> ApiResult<LoginSuccess> {
    let info = data.into_inner();
    let (auth, refresh, user) = Auth::login(
        &*conn,
        &info.email,
        &info.password,
        &state.access_lifetime,
        &state.refresh_lifetime,
    )?;

    Ok(Json(LoginSuccess {
        access_token: auth.token(&state.jwt_secret)?,
        refresh_token: refresh.hash,
        user: user.data(),
    }))
}

#[post("/api/v1/auth/logout", format = "json", data = "<data>")]
fn logout(conn: DBConnection, data: Json<LogoutData>) -> ApiResult<()> {
    let LogoutData {
        email,
        refresh_token,
    } = data.into_inner();
    Auth::logout(&*conn, &email, &refresh_token)?;

    OK()
}

/// Activate a user
///
/// The calls to the activation are generated by the application and the client should
/// never be able to create a valid activation link on his own.
///
/// A call to this web service completes the registration process and allows a user to
/// log in the application.
#[post("/api/v1/auth/activate", format = "json", data = "<data>")]
fn activate(conn: DBConnection, state: State, data: Json<ActivationData>) -> ApiResult<()> {
    let ActivationData { id, token } = data.into_inner();

    let mut user: UserEntity =
        UserEntity::by_id(&*conn, &id)?.map_or_else(|| Err(AuthError::InvalidIDs), |v| Ok(v))?;
    if user.active {
        Err(AuthError::AlreadyActivated)?;
    }

    let mut activation_token = user
        .activation_token(&*conn)?
        .map_or_else(|| Err(AuthError::InvalidToken), |v| Ok(v))?;

    activation_token.vouch(&conn, &token)?;

    let recovery_token = TokenEntity::create_default(&*conn)?;
    let refresh_token = TokenEntity::create(&*conn, Some(&state.access_lifetime), Some(&-1))?;

    user.recovery_token = Some(recovery_token.id);
    user.refresh_token = Some(refresh_token.id);
    user.active = true;
    user.update(&*conn)?;

    OK()
}

#[post("/api/v1/auth/restore", format = "json", data = "<data>")]
fn restore(conn: DBConnection, data: Json<RestoreData>) -> ApiResult<()> {
    let email = data.into_inner().email;

    let mut user = UserEntity::by_email(&*conn, &email)??;
    let mut recovery_token = user
        .recovery_token(&*conn)?
        .map_or_else(|| Err(AuthError::InvalidToken), |v| Ok(v))?;
    recovery_token.renew(&conn, Some(&(3600)), Some(&1))?;
    user.recovery_token = Some((&recovery_token).id);
    user.update(&*conn)?;

    mail::send(
        &user,
        &format!("unanimity.be/recover/{}/{}", &user.id, &recovery_token.hash),
        &vec![],
    )?;

    OK()
}

#[post("/api/v1/auth/recover", format = "json", data = "<data>")]
fn recover(conn: DBConnection, data: Json<RecoveryData>) -> ApiResult<()> {
    let RecoveryData {
        id,
        password,
        token,
    } = data.into_inner();

    let mut user = UserEntity::by_id(&*conn, &id)??;
    let mut recovery_token = user
        .recovery_token(&*conn)?
        .map_or_else(|| Err(AuthError::InvalidToken), |v| Ok(v))?;

    recovery_token.vouch(&conn, &token)?;

    user.set_password(&password)?;
    user.update(&conn)?;

    OK()
}

#[post("/api/v1/auth/refresh", format = "json", data = "<data>")]
fn refresh(conn: DBConnection, state: State, data: Json<RefreshData>) -> ApiResult<RefreshSuccess> {
    let RefreshData {
        email,
        refresh_token,
    } = data.into_inner();
    let (auth, token, user) = Auth::refresh(
        &*conn,
        &email,
        &refresh_token,
        &state.access_lifetime,
        &state.refresh_lifetime,
    )?;

    Ok(Json(RefreshSuccess {
        access_token: auth.token(&state.jwt_secret)?,
        user: user.data(),
        refresh_token: token.hash,
    }))
}