//! Error types for the integrations crate.

use thiserror::Error;

/// Errors that can occur when working with integrations.
#[derive(Error, Debug)]
pub enum IntegrationError {
    /// The specified integration was not found.
    #[error("Integration not found: {0}")]
    IntegrationNotFound(String),

    /// The specified function was not found.
    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    /// The function execution failed.
    #[error("Function execution failed: {0} - {1}")]
    FunctionExecutionFailed(String, String),

    /// Error communicating with the integration server.
    #[error("Communication error: {0}")]
    CommunicationError(#[from] reqwest::Error),

    /// Error parsing JSON.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Database error.
    #[error("Database error: {0}")]
    DatabaseError(String),
}
