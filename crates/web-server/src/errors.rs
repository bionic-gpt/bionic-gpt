use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
    ExternalApi(String),
    Authentication(String),
    Authorization,
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::ExternalApi(ref cause) => write!(f, "Api Error: {}", cause),
            CustomError::Authentication(ref cause) => write!(f, "Api Error: {}", cause),
            CustomError::Authorization => write!(f, "Authorization Error"),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {}", cause)
            }
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::ExternalApi(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::Authentication(message) => (StatusCode::UNAUTHORIZED, message),
            CustomError::Authorization => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        let response = format!("status = {}, message = {}", status, error_message);

        tracing::error!(response);

        response.into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<http::Error> for CustomError {
    fn from(err: http::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<db::TokioPostgresError> for CustomError {
    fn from(err: db::TokioPostgresError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<db::PoolError> for CustomError {
    fn from(err: db::PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
