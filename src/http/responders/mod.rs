pub mod api;
pub mod catcher;
pub mod error;

use rocket_contrib::json::Json;
use crate::lib::Consequence;

pub type ApiResult<T> = Consequence<Json<T>>;

pub fn OK() -> ApiResult<()> { Ok(Json(())) }

