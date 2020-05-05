use rocket::Request;
use rocket::http::RawStr;
use rocket::request::{FromParam, FromFormValue};
use std::ops::Deref;
pub mod quick_response;

#[derive(Debug)]
pub enum RequestType {
    Vanilla,
    Json,
    Unknown,
}

impl RequestType {
    pub fn guess(req: &Request) -> Self {
        // check if the url path begins with /api/
        let (begin, _) = req.uri().path().split_at(5);
        if begin == "/api/" {
            RequestType::Json
        } else {
            RequestType::Vanilla
        }
    }
}


// Need a new type as Vec<String> is an extern struct
pub struct StringVector(Vec<String>);

impl Deref for StringVector {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'r> FromParam<'r> for StringVector {
    type Error = ();

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        Ok(StringVector(param
            .percent_decode()
            .map_err(|_| ())?
            .split(":")
            .map(|v| v.to_string())
            .collect::<Vec<String>>()))
    }
}

impl<'r> FromFormValue<'r> for StringVector {
    type Error = ();

    fn from_form_value(form_value: &'r RawStr) -> Result<Self, Self::Error> {
        Ok(StringVector(form_value
            .percent_decode()
            .map_err(|_| ())?
            .split(":")
            .map(|v| v.to_string())
            .collect::<Vec<String>>()))
    }

    fn default() -> Option<Self> {
        Some(StringVector(vec!()))
    }
}