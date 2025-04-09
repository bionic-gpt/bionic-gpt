//! Integration framework for external tools and services.

mod error;
mod integrations;
pub mod models;
mod registry;
#[cfg(test)]
mod registry_test;
mod tools;

pub use tools::{execute_tool_call, get_tools, Registry};

pub use error::IntegrationError;
pub use models::{
    Completion, FunctionDefinition, Message, Tool, ToolCall, ToolCallFunction, ToolResult,
};
pub use registry::IntegrationRegistry;

use async_trait::async_trait;

/// Trait that all integrations must implement.
#[async_trait]
pub trait Integration: Send + Sync {
    /// Get the name of the integration.
    fn name(&self) -> &str;

    /// Get the description of the integration.
    fn description(&self) -> &str;

    /// Discover the tools provided by this integration.
    async fn discover(&self) -> Result<Vec<models::Tool>, IntegrationError>;

    /// Execute a function with the given arguments.
    async fn execute(
        &self,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, IntegrationError>;
}
