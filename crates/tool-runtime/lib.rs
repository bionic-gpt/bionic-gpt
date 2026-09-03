//! Tool runtime crate.
//!
//! This crate defines tool interfaces, tool catalogs, tool dispatch,
//! and OpenAPI-backed tool adapters used by the agent runtime.

pub mod builtin_tools;
pub mod openapi_tool_factory;
pub mod scheduled_tasks;
pub mod skills;
pub mod system_tool_sources;
pub mod token_count;
pub mod tool_auth;
pub mod tool_catalog;
mod tool_contract;
pub mod tool_dispatcher;
pub mod types;

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
pub use openapi_tool_factory::{BionicOpenAPI, IntegrationTools, OAuth2Config};
pub use tool_auth::{OAuth2TokenProvider, StaticTokenProvider, TokenProvider};
pub use tool_catalog::get_chat_tool_definitions;
pub use tool_contract::{ToolDyn, ToolError};
pub use tool_dispatcher::{execute_tool_call_with_tools, execute_tool_calls};
pub use types::{
    parse_reasoning, parse_tool_calls, serialize_assistant_tool_state, Reasoning,
    StoredAssistantToolState, ToolCall, ToolCallFunction, ToolDefinition, ToolResult,
    ToolResultContent,
};
