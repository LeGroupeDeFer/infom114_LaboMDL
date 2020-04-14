pub mod api;
pub mod catcher;
pub mod error;

use crate::database::models::prelude::Result;
use rocket_contrib::json::Json;

pub type ApiResult<T> = Result<Json<T>>;

pub fn OK() -> ApiResult<()> { Ok(Json(())) }
