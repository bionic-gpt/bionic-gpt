use super::CustomError;
use axum::response::{IntoResponse, Redirect};
use serde::{Deserialize, Deserializer};

pub fn redirect_and_snackbar(
    url: &str,
    _message: &'static str,
) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(url))
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
