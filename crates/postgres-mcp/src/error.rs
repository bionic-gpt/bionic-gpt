use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;
use std::{fmt, io};
use tracing::error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<Value>,
}

impl ApiError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        message: impl Into<String>,
        details: Option<Value>,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message, None)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            message,
            None,
        )
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message, None)
    }

    pub fn database_error(message: impl Into<String>, details: Option<Value>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database_error",
            message,
            details,
        )
    }

    fn as_response(&self) -> ErrorResponse<'_> {
        ErrorResponse {
            error: ErrorDetail {
                code: self.code,
                message: &self.message,
                details: self.details.as_ref(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.status.is_server_error() {
            error!(
                code = self.code,
                message = %self.message,
                details = ?self.details,
                "request failed"
            );
        }

        let status = self.status;
        let body = Json(self.as_response());
        (status, body).into_response()
    }
}

impl From<hyper::Error> for ApiError {
    fn from(err: hyper::Error) -> Self {
        ApiError::internal(format!("server error: {err}"))
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        ApiError::internal(format!("io error: {err}"))
    }
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(err: tokio_postgres::Error) -> Self {
        if let Some(db_err) = err.as_db_error() {
            let sqlstate = db_err.code().code();
            if sqlstate.starts_with("28") {
                return ApiError::unauthorized(db_err.message().to_string());
            }

            let mut details = serde_json::Map::new();
            details.insert(
                "severity".into(),
                Value::String(db_err.severity().to_string()),
            );
            details.insert("sqlstate".into(), Value::String(sqlstate.to_string()));
            if let Some(schema) = db_err.schema() {
                details.insert("schema".into(), Value::String(schema.to_string()));
            }
            if let Some(table) = db_err.table() {
                details.insert("table".into(), Value::String(table.to_string()));
            }
            if let Some(column) = db_err.column() {
                details.insert("column".into(), Value::String(column.to_string()));
            }

            return ApiError::database_error(
                db_err.message().to_string(),
                Some(Value::Object(details)),
            );
        }

        if err.to_string().contains("password authentication failed") {
            return ApiError::unauthorized("authentication failed");
        }

        ApiError::database_error(format!("database error: {err}"), None)
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    error: ErrorDetail<'a>,
}

#[derive(Debug, Serialize)]
struct ErrorDetail<'a> {
    code: &'a str,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for ApiError {}
