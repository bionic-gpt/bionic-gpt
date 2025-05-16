//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod attachment_as_text;
pub mod attachment_to_markdown;
pub mod attachments_list;
pub mod external_integration;
pub mod open_api_v3;
pub mod time_date;
pub mod tool;
pub mod tool_executor;
pub mod tool_registry;

#[cfg(test)]
mod test_async;

// Re-export key types for convenience
pub use external_integration::{create_external_integration_tools, ExternalIntegrationTool};
pub use tool::ToolInterface;
pub use tool_executor::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_registry::{
    get_chat_tools_user_selected, get_integrations, get_tools_for_attachments,
    get_user_selectable_tools_for_chat_ui, IntegrationTool, ToolScope,
};
