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
use openai::chat::ChatCompletionFunctionDefinition;
use serde::{Deserialize, Serialize};
pub use tool::ToolInterface;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct BionicToolDefinition {
    pub r#type: String,
    /// The function that the model called.
    pub function: ChatCompletionFunctionDefinition,
}
