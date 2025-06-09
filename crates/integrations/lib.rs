//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod bionic_openapi;
pub mod open_api_tool;
pub mod tool;
pub mod tool_executor;
pub mod tool_registry;
pub mod tools;

#[cfg(test)]
mod test_async;

/// Create a JSON error object with a message and details
pub fn json_error(kind: &str, err: impl ToString) -> serde_json::Value {
    serde_json::json!({
        "error": kind,
        "details": err.to_string(),
    })
}

// Re-export key types for convenience
pub use bionic_openapi::{
    create_tools_from_integration, create_tools_from_integrations, BionicOpenAPI, IntegrationTools,
    OAuth2Config,
};
pub use open_api_tool::OpenApiTool;
pub use tool::ToolInterface;
pub use tool_executor::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_registry::{
    get_chat_tools_user_selected, get_integrations, get_tools, IntegrationTool, ToolScope,
};
