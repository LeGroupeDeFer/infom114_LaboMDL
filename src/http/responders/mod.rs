pub mod api;
pub mod catcher;
pub mod error;

use crate::lib::Consequence;
use rocket_contrib::json::Json;

pub type ApiResult<T> = Consequence<Json<T>>;

pub fn ok() -> ApiResult<()> {
    Ok(Json(()))
}
