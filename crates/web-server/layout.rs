use super::CustomError;
use axum::http::header::SET_COOKIE;
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Redirect};
use serde::{Deserialize, Deserializer};

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<impl IntoResponse, CustomError> {
    let mut response = Redirect::to(url).into_response();
    let cookie_value = format!("flash_aargh={}; Path=/", message);
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
    Ok(response)
}

pub fn redirect(url: &str) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(url))
}

pub fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
