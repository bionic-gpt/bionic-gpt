use super::CustomError;
use axum::response::{IntoResponse, Redirect};

pub fn redirect_and_snackbar(
    url: &str,
    _message: &'static str,
) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(url))
}

pub fn redirect(url: &str) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(url))
}
