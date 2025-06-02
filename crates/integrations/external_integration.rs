use crate::tool::ToolInterface;
use async_trait::async_trait;
use oas3::{self, spec::Operation};

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
    for (_path, _method, operation) in spec.operations() {
        if let Some(operation_id) = &operation.operation_id {
            let mut parameters = None;
            let function_name = operation_id.clone();
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

        let integration_tools = create_tool_definitions_from_spec(oas3.clone());
        let base_url = integration_tools
            .base_url
            .unwrap_or_else(|| "http://localhost".to_string());

        // Create tools for each tool definition
        for tool_def in integration_tools.tool_definitions {
            // The function name is now the operation_id, so we use it directly
            let operation_id = tool_def.function.name.clone();

            let tool = ExternalIntegrationTool::new(
                tool_def,
                base_url.clone(),
                oas3.clone(),
                operation_id,
            );
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
    /// The OpenAPI specification
    spec: oas3::OpenApiV3Spec,
    /// The operation ID for this tool
    operation_id: String,
}

impl ExternalIntegrationTool {
    pub fn new(
        definition: BionicToolDefinition,
        base_url: String,
        spec: oas3::OpenApiV3Spec,
        operation_id: String,
    ) -> Self {
        Self {
            definition,
            base_url,
            client: Client::new(),
            spec,
            operation_id,
        }
    }

    /// Find operation details by operation_id in the OpenAPI spec
    fn find_operation_details(&self) -> Result<(String, String, &Operation), String> {
        for (path, method, operation) in self.spec.operations() {
            if let Some(op_id) = &operation.operation_id {
                if op_id == &self.operation_id {
                    return Ok((path.to_string(), method.to_string(), operation));
                }
            }
        }
        Err(format!(
            "Operation with ID '{}' not found in OpenAPI spec",
            self.operation_id
        ))
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

        // Find operation details by operation_id
        let (path, method, _operation) = self.find_operation_details()?;

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Failed to parse arguments: {}", e))?;

        // Construct the URL
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("Making request to URL: {} using method: {}", url, method);

        // Make the request based on the HTTP method
        let response = match method.to_uppercase().as_str() {
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
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_openapi_spec() -> oas3::OpenApiV3Spec {
        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "servers": [
                {
                    "url": "https://api.example.com"
                }
            ],
            "paths": {
                "/users": {
                    "get": {
                        "operationId": "getUsers",
                        "summary": "Get all users",
                        "description": "Retrieve a list of all users"
                    },
                    "post": {
                        "operationId": "createUser",
                        "summary": "Create a user",
                        "description": "Create a new user"
                    }
                },
                "/users/{id}": {
                    "get": {
                        "operationId": "getUserById",
                        "summary": "Get user by ID",
                        "description": "Retrieve a specific user by ID"
                    }
                }
            }
        });

        serde_json::from_value(spec_json).unwrap()
    }

    #[test]
    fn test_create_tool_definitions_uses_operation_id() {
        let spec = create_test_openapi_spec();
        let integration_tools = create_tool_definitions_from_spec(spec);

        assert_eq!(integration_tools.tool_definitions.len(), 3);

        let tool_names: Vec<String> = integration_tools
            .tool_definitions
            .iter()
            .map(|t| t.function.name.clone())
            .collect();

        assert!(tool_names.contains(&"getUsers".to_string()));
        assert!(tool_names.contains(&"createUser".to_string()));
        assert!(tool_names.contains(&"getUserById".to_string()));
    }

    #[test]
    fn test_extract_base_url() {
        let spec = create_test_openapi_spec();
        let base_url = extract_base_url(&spec);
        assert_eq!(base_url, Some("https://api.example.com".to_string()));
    }

    #[test]
    fn test_external_integration_tool_find_operation_details() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "getUsers".to_string(),
                description: Some("Get all users".to_string()),
                parameters: None,
            },
        };

        let tool = ExternalIntegrationTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "getUsers".to_string(),
        );

        let result = tool.find_operation_details();
        assert!(result.is_ok());

        let (path, method, operation) = result.unwrap();
        assert_eq!(path, "/users");
        assert_eq!(method, "GET");
        assert_eq!(operation.operation_id, Some("getUsers".to_string()));
    }

    #[test]
    fn test_external_integration_tool_operation_not_found() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "nonExistentOperation".to_string(),
                description: Some("Non-existent operation".to_string()),
                parameters: None,
            },
        };

        let tool = ExternalIntegrationTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "nonExistentOperation".to_string(),
        );

        let result = tool.find_operation_details();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Operation with ID 'nonExistentOperation' not found"));
    }

    #[test]
    fn test_tool_name_returns_operation_id() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "createUser".to_string(),
                description: Some("Create a user".to_string()),
                parameters: None,
            },
        };

        let tool = ExternalIntegrationTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "createUser".to_string(),
        );

        assert_eq!(tool.name(), "createUser");
    }
}
