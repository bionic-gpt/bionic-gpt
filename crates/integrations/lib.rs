//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod attachments;
pub mod time_date;
pub mod tool;
pub mod tool_executor;
pub mod tool_registry;

// Re-export key types for convenience
pub use tool::ToolInterface;
pub use tool_executor::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_registry::{get_chat_tools_user_selected, get_user_selectable_tools_for_chat};
