//! Tool runtime crate.
//!
//! This crate defines tool interfaces, tool catalogs, tool dispatch,
//! and OpenAPI-backed tool adapters used by the agent runtime.

pub mod builtin_tools;
pub mod openapi_tool_factory;
pub mod system_tool_sources;
pub mod tool_auth;
pub mod tool_catalog;
pub mod tool_dispatcher;
pub mod tool_interface;

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
pub use builtin_tools::openapi_tool_adapter::OpenApiTool;
pub use openapi_tool_factory::{
    create_tools_from_integration, create_tools_from_integrations, BionicOpenAPI, IntegrationTools,
    OAuth2Config,
};
pub use system_tool_sources::{get_system_openapi_tool_definitions, get_system_openapi_tools};
pub use tool_auth::{OAuth2TokenProvider, StaticTokenProvider, TokenProvider};
pub use tool_catalog::{
    get_chat_tools_user_selected, get_chat_tools_user_selected_with_system_openapi,
    get_integrations, get_tools, get_tools_with_system_openapi, IntegrationTool, ToolScope,
};
pub use tool_dispatcher::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_interface::ToolInterface;
