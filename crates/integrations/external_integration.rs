use crate::tool::ToolInterface;
use async_trait::async_trait;
use oas3;

use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

/// Result of parsing an OpenAPI specification for tool creation
pub struct IntegrationTools {
    pub tool_definitions: Vec<BionicToolDefinition>,
    pub base_url: Option<String>,
}

/// Extract the base URL from the first server in the OpenAPI specification
pub fn extract_base_url(oas3: &oas3::OpenApiV3Spec) -> Option<String> {
    if !oas3.servers.is_empty() {
        return Some(oas3.servers[0].url.clone());
    }
    None
}

/// Create tool definitions from an OpenAPI specification
pub fn create_tool_definitions_from_spec(spec: oas3::OpenApiV3Spec) -> IntegrationTools {
    let mut tool_definitions: Vec<BionicToolDefinition> = vec![];
    let base_url = extract_base_url(&spec);

    // Process each operation in the OpenAPI spec
    for (path, _method, operation) in spec.operations() {
        if let Some(_operation_id) = &operation.operation_id {
            let mut parameters = None;
            let function_name = path.replace('/', "");
            let schema_key = format!("{}_form_model", function_name);

            // Try to get schema-based parameters from components
            if let Some(components) = &spec.components {
                let schema = components.schemas.get(&schema_key);
                if let Some(schema) = schema {
                    let params = serde_json::to_value(schema).unwrap_or_default();
                    parameters = Some(params);
                }
            }

            let definition = BionicToolDefinition {
                r#type: "function".to_string(),
                function: ChatCompletionFunctionDefinition {
                    name: function_name,
                    description: operation
                        .description
                        .clone()
                        .or_else(|| operation.summary.clone()),
                    parameters,
                },
            };

            tool_definitions.push(definition);
        }
    }

    IntegrationTools {
        tool_definitions,
        base_url,
    }
}

/// Create tools from a single integration
pub fn create_tools_from_integration(
    integration: &db::queries::integrations::Integration,
) -> Result<Vec<Arc<dyn ToolInterface>>, String> {
    let mut tools: Vec<Arc<dyn ToolInterface>> = Vec::new();

    if let Some(definition) = &integration.definition {
        let oas3 = oas3::from_json(definition.to_string())
            .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

        let integration_tools = create_tool_definitions_from_spec(oas3);
        let base_url = integration_tools
            .base_url
            .unwrap_or_else(|| "http://localhost".to_string());

        // Create tools for each tool definition
        for tool_def in integration_tools.tool_definitions {
            // For now, we'll use POST method and the function name as path
            // This is a simplified approach - in a real implementation you might
            // want to store method and path information separately
            let path = format!("/{}", tool_def.function.name);
            let method = "POST".to_string();

            let tool = ExternalIntegrationTool::new(tool_def, base_url.clone(), path, method);
            tools.push(Arc::new(tool));
        }
    } else {
        return Err("Integration doesn't have a definition".to_string());
    }

    Ok(tools)
}

/// A tool that executes external integrations based on OpenAPI definitions
pub struct ExternalIntegrationTool {
    /// The tool definition
    definition: BionicToolDefinition,
    /// The base URL for the API
    base_url: String,
    /// The HTTP client
    client: Client,
    /// The path for this operation
    path: String,
    /// The HTTP method for this operation
    method: String,
}

impl ExternalIntegrationTool {
    pub fn new(
        definition: BionicToolDefinition,
        base_url: String,
        path: String,
        method: String,
    ) -> Self {
        Self {
            definition,
            base_url,
            client: Client::new(),
            path,
            method,
        }
    }
}

#[async_trait]
impl ToolInterface for ExternalIntegrationTool {
    fn get_tool(&self) -> BionicToolDefinition {
        self.definition.clone()
    }

    async fn execute(&self, arguments: &str) -> Result<String, String> {
        tracing::info!(
            "Executing external integration tool {} with arguments: {}",
            self.name(),
            arguments
        );

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Failed to parse arguments: {}", e))?;

        // Construct the URL
        let url = format!("{}{}", self.base_url, self.path);
        tracing::debug!("Making request to URL: {}", url);

        // Make the request based on the HTTP method
        let response = match self.method.to_uppercase().as_str() {
            "GET" => self
                .client
                .get(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make GET request: {}", e))?,
            "POST" => self
                .client
                .post(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make POST request: {}", e))?,
            "PUT" => self
                .client
                .put(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make PUT request: {}", e))?,
            "DELETE" => self
                .client
                .delete(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make DELETE request: {}", e))?,
            _ => return Err(format!("Unsupported HTTP method: {}", self.method)),
        };

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(format!("Request failed with status: {}", response.status()));
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        Ok(response_text)
    }
}

/// Create tools from integrations
pub async fn create_tools_from_integrations(
    integrations: Vec<db::queries::integrations::Integration>,
) -> Vec<Arc<dyn ToolInterface>> {
    let mut tools: Vec<Arc<dyn ToolInterface>> = Vec::new();

    for integration in integrations {
        match create_tools_from_integration(&integration) {
            Ok(integration_tools) => {
                tools.extend(integration_tools);
            }
            Err(error) => {
                tracing::error!(
                    "Failed to create tools for integration {}: {}",
                    integration.id,
                    error
                );
            }
        }
    }

    tools
}
