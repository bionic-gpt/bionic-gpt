//! Tool interface definition
//!
//! This module defines the ToolInterface trait that all tools must implement.

use async_trait::async_trait;
use openai_api::BionicToolDefinition;

/// Tool interface trait that defines the common functionality for all tools
#[async_trait]
pub trait ToolInterface: Send + Sync {
    /// Returns the tool definition
    fn get_tool(&self) -> BionicToolDefinition;

    /// Executes the tool with the given arguments
    async fn execute(&self, arguments: &str) -> Result<String, String>;

    /// Returns the name of the tool
    fn name(&self) -> String {
        self.get_tool().function.name.clone()
    }
}
