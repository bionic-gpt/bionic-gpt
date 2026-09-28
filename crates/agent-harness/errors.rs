use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::backtrace::Backtrace;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String, Backtrace),
    ExternalApi(String),
    Authentication(String),
    Limits(String),
    Authorization,
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::ExternalApi(ref cause) => write!(f, "Api Error: {}", cause),
            CustomError::Authentication(ref cause) => write!(f, "Api Error: {}", cause),
            CustomError::Limits(ref cause) => write!(f, "Api Error: {}", cause),
            CustomError::Authorization => write!(f, "Authorization Error"),
            CustomError::Database(ref cause, ref backtrace) => {
                write!(f, "Database Error: {}\nBacktrace:\n{}", cause, backtrace)
            }
        }
    }
}

// Implement std::error::Error trait for CustomError
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message, backtrace_info) = match self {
            CustomError::Database(message, backtrace) => {
                let backtrace_str = format!("{}", backtrace);
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                    Some(backtrace_str),
                )
            }
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message, None),
            CustomError::ExternalApi(message) => (StatusCode::UNPROCESSABLE_ENTITY, message, None),
            CustomError::Authentication(message) => (StatusCode::UNAUTHORIZED, message, None),
            CustomError::Limits(message) => (StatusCode::TOO_MANY_REQUESTS, message, None),
            CustomError::Authorization => {
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None)
            }
        };

        let response_body = format!("status = {}, message = {}", status, error_message);

        // Log the error with backtrace if available
        if let Some(backtrace) = backtrace_info {
            tracing::error!(
                response_body = %response_body,
                backtrace = %backtrace,
                "Database error with stack trace"
            );
        } else {
            tracing::error!(response_body);
        }

        (status, response_body).into_response()
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
        CustomError::Database(err.to_string(), Backtrace::capture())
    }
}

impl From<db::PoolError> for CustomError {
    fn from(err: db::PoolError) -> CustomError {
        CustomError::Database(err.to_string(), Backtrace::capture())
    }
}

impl From<CustomError> for axum::Error {
    fn from(err: CustomError) -> axum::Error {
        axum::Error::new(err)
    }
}
