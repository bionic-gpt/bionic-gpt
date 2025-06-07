//! OpenAPI v3 Integration
//!
//! This module provides the main public API for working with OpenAPI v3 specifications
//! and creating tools from integrations.

use crate::tool::ToolInterface;
use openai_api::BionicToolDefinition;
use std::sync::Arc;

// Re-export the new modules
pub use crate::bionic_openapi::BionicOpenAPI;
pub use crate::open_api_tool::OpenApiTool;

/// Result of parsing an OpenAPI specification for tool creation
pub struct IntegrationTools {
    pub tool_definitions: Vec<BionicToolDefinition>,
    pub base_url: Option<String>,
}

/// Extract the base URL from the first server in the OpenAPI specification
pub fn extract_base_url(oas3: &oas3::OpenApiV3Spec) -> Option<String> {
    let bionic_api = crate::bionic_openapi::BionicOpenAPI::new(oas3.clone());
    bionic_api.extract_base_url()
}

/// Create tool definitions from an OpenAPI specification
pub fn create_tool_definitions_from_spec(spec: oas3::OpenApiV3Spec) -> IntegrationTools {
    let bionic_api = crate::bionic_openapi::BionicOpenAPI::new(spec);
    bionic_api.create_tool_definitions()
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

            let tool = crate::open_api_tool::OpenApiTool::new(
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

    fn create_numeric_boolean_spec() -> oas3::OpenApiV3Spec {
        let spec_json = json!({
            "openapi": "3.0.3",
            "info": {"title": "Numeric and Boolean", "version": "1.0"},
            "paths": {
                "/items": {
                    "get": {
                        "operationId": "getItems",
                        "parameters": [
                            {"in": "query", "name": "limit", "required": true, "schema": {"type": "integer"}},
                            {"in": "query", "name": "active", "required": false, "schema": {"type": "boolean"}}
                        ],
                        "responses": {"200": {"description": "ok"}}
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
    fn test_numeric_and_boolean_parameter_types() {
        let spec = create_numeric_boolean_spec();
        let integration_tools = create_tool_definitions_from_spec(spec);

        let tool = integration_tools
            .tool_definitions
            .iter()
            .find(|t| t.function.name == "getItems")
            .expect("getItems tool should exist");

        let params = tool
            .function
            .parameters
            .as_ref()
            .expect("parameters should be present");

        assert_eq!(params["properties"]["limit"]["type"], "integer");
        assert_eq!(params["properties"]["active"]["type"], "boolean");
    }
}
