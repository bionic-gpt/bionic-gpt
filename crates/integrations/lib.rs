//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod attachments_list;
pub mod attachments_read;
pub mod time_date;
pub mod tool;
pub mod tool_executor;
pub mod tool_registry;

#[cfg(test)]
mod test_async;

// Re-export key types for convenience
pub use tool::ToolInterface;
pub use tool_executor::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_registry::{
    get_all_integrations, get_chat_tools_user_selected, get_tools_for_attachments,
    get_user_selectable_tools_for_chat_ui, IntegrationTool,
};
