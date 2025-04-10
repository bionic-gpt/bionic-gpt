//! Tool interface definition
//!
//! This module defines the ToolInterface trait that all tools must implement.

use openai_api::Tool;

/// Tool interface trait that defines the common functionality for all tools
pub trait ToolInterface: Send + Sync {
    /// Returns the tool definition
    fn get_tool(&self) -> Tool;

    /// Executes the tool with the given arguments
    fn execute(&self, arguments: &str) -> Result<String, String>;

    /// Returns the name of the tool
    fn name(&self) -> String {
        self.get_tool().function.name.clone()
    }
}
