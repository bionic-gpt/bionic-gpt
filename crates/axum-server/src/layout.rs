use crate::errors::CustomError;
use axum::http::Response;
use axum::response::IntoResponse;
use hyper::{Body, StatusCode};

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<impl IntoResponse, CustomError> {
    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", url)
        .header("set-cookie", format!("flash_aargh={}; Max-Age=6", message))
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}
