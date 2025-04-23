//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod function_executor;
pub mod function_tools;
pub mod tool;
pub mod weather;

// Re-export key types for convenience
pub use function_executor::{execute_tool_call, execute_tool_call_with_tools};
pub use function_tools::get_tools;
pub use tool::ToolInterface;
