//! Tools for integrating with LLM models.

use crate::models::{Message, Tool, ToolCall};
use async_trait::async_trait;
use std::env;

/// Trait for registry-like objects that can discover tools and execute functions.
#[async_trait]
pub trait Registry {
    /// Discover all tools from all registered integrations.
    async fn discover_all(&self) -> Vec<Tool>;

    /// Execute a function on an integration.
    async fn execute(
        &self,
        integration_name: &str,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, crate::IntegrationError>;
}
#[async_trait]
impl Registry for crate::IntegrationRegistry {
    async fn discover_all(&self) -> Vec<Tool> {
        crate::IntegrationRegistry::discover_all(self).await
    }

    async fn execute(
        &self,
        integration_name: &str,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, crate::IntegrationError> {
        crate::IntegrationRegistry::execute(self, integration_name, function_name, arguments).await
    }
}

/// Returns a list of available tools from all registered integrations.
/// Only returns tools if the DANGER_JWT_OVERRIDE environment variable is set.
pub async fn get_tools<R: Registry + ?Sized>(registry: Option<&R>) -> Vec<Tool> {
    // Check if DANGER_JWT_OVERRIDE environment variable is set
    match env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => {
            let mut tools = Vec::new();

            // Add tools from integrations if registry is provided
            if let Some(registry) = registry {
                let integration_tools = registry.discover_all().await;
                tools.extend(integration_tools);
            }

            tools
        }
        Err(_) => vec![], // Return empty vector if DANGER_JWT_OVERRIDE is not set
    }
}

/// Execute a tool call and return a message with the result.
pub async fn execute_tool_call<R: Registry + ?Sized>(
    registry: Option<&R>,
    tool_call: &ToolCall,
) -> Result<Message, String> {
    // Check if this is an integration function
    if let Some(registry) = registry {
        // Parse the function name to get the integration name and function name
        // Format: "integration_name.function_name"
        let parts: Vec<&str> = tool_call.function.name.split('.').collect();
        if parts.len() == 2 {
            let integration_name = parts[0];
            let function_name = parts[1];

            match registry
                .execute(
                    integration_name,
                    function_name,
                    &tool_call.function.arguments,
                )
                .await
            {
                Ok(result) => {
                    return Ok(Message {
                        role: "tool".to_string(),
                        content: result,
                        tool_call_id: Some(tool_call.id.clone()),
                        name: Some(tool_call.function.name.clone()),
                        tool_calls: None,
                    });
                }
                Err(err) => {
                    return Err(format!("Error executing function: {}", err));
                }
            }
        }
    }

    Err(format!("Unknown function: {}", tool_call.function.name))
}
