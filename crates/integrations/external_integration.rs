use crate::tool::ToolInterface;
use async_trait::async_trait;
use oas3::{self, spec::Operation};

use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

/// Create a JSON error object with a message and details
fn json_error(kind: &str, err: impl ToString) -> serde_json::Value {
    serde_json::json!({
        "error": kind,
        "details": err.to_string(),
    })
}

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

/// Extract parameters from an OpenAPI operation and convert to JSON Schema format
fn extract_operation_parameters(operation: &Operation) -> Option<Value> {
    if operation.parameters.is_empty() {
        return None;
    }

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    // For now, let's create a simple implementation that works with the basic case
    // We'll iterate through parameters and create a basic JSON schema
    for param in &operation.parameters {
        // Try to extract parameter information using serde_json serialization
        if let Ok(param_value) = serde_json::to_value(param) {
            if let Some(param_obj) = param_value.as_object() {
                if let Some(name) = param_obj.get("name").and_then(|n| n.as_str()) {
                    let mut property = serde_json::Map::new();
                    property.insert("type".to_string(), Value::String("string".to_string()));

                    if let Some(description) = param_obj.get("description").and_then(|d| d.as_str())
                    {
                        property.insert(
                            "description".to_string(),
                            Value::String(description.to_string()),
                        );
                    }

                    properties.insert(name.to_string(), Value::Object(property));

                    // Check if required
                    if param_obj
                        .get("required")
                        .and_then(|r| r.as_bool())
                        .unwrap_or(false)
                    {
                        required.push(name.to_string());
                    }
                }
            }
        }
    }

    if properties.is_empty() {
        return None;
    }

    // Build the JSON Schema object
    let mut schema = serde_json::Map::new();
    schema.insert("type".to_string(), Value::String("object".to_string()));
    schema.insert("properties".to_string(), Value::Object(properties));

    if !required.is_empty() {
        schema.insert(
            "required".to_string(),
            Value::Array(required.into_iter().map(Value::String).collect()),
        );
    }

    Some(Value::Object(schema))
}

/// Merge schema-based parameters with operation-based parameters
/// Operation parameters take precedence over schema parameters
fn merge_parameters(
    schema_params: Option<Value>,
    operation_params: Option<Value>,
) -> Option<Value> {
    match (schema_params, operation_params) {
        (None, None) => None,
        (Some(schema), None) => Some(schema),
        (None, Some(operation)) => Some(operation),
        (Some(mut schema), Some(operation)) => {
            // Merge operation parameters into schema parameters
            // Operation parameters take precedence
            if let (Value::Object(ref mut schema_obj), Value::Object(operation_obj)) =
                (&mut schema, &operation)
            {
                // Merge properties
                if let (
                    Some(Value::Object(ref mut schema_props)),
                    Some(Value::Object(operation_props)),
                ) = (
                    schema_obj.get_mut("properties"),
                    operation_obj.get("properties"),
                ) {
                    for (key, value) in operation_props {
                        schema_props.insert(key.clone(), value.clone());
                    }
                }

                // Merge required fields
                if let Some(Value::Array(operation_required)) = operation_obj.get("required") {
                    let schema_required = schema_obj
                        .entry("required")
                        .or_insert_with(|| Value::Array(Vec::new()));
                    if let Value::Array(ref mut schema_req_array) = schema_required {
                        for req in operation_required {
                            if !schema_req_array.contains(req) {
                                schema_req_array.push(req.clone());
                            }
                        }
                    }
                }
            }
            Some(schema)
        }
    }
}

/// Substitute path parameters in a URL template with actual values
fn substitute_path_parameters(
    path: &str,
    args: &Value,
    operation: &Operation,
) -> Result<String, String> {
    let mut result_path = path.to_string();

    // Extract path parameters from the operation
    for param in &operation.parameters {
        if let Ok(param_value) = serde_json::to_value(param) {
            if let Some(param_obj) = param_value.as_object() {
                // Check if this is a path parameter
                if let Some(location) = param_obj.get("in").and_then(|l| l.as_str()) {
                    if location == "path" {
                        if let Some(param_name) = param_obj.get("name").and_then(|n| n.as_str()) {
                            let placeholder = format!("{{{}}}", param_name);

                            if let Some(value) = args.get(param_name) {
                                let value_str = match value {
                                    Value::String(s) => s.clone(),
                                    Value::Number(n) => n.to_string(),
                                    Value::Bool(b) => b.to_string(),
                                    _ => {
                                        return Err(format!(
                                            "Invalid value type for path parameter: {}",
                                            param_name
                                        ))
                                    }
                                };
                                result_path = result_path.replace(&placeholder, &value_str);
                            } else {
                                // Check if parameter is required
                                let is_required = param_obj
                                    .get("required")
                                    .and_then(|r| r.as_bool())
                                    .unwrap_or(false);
                                if is_required {
                                    return Err(format!(
                                        "Missing required path parameter: {}",
                                        param_name
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(result_path)
}

/// Create tool definitions from an OpenAPI specification
pub fn create_tool_definitions_from_spec(spec: oas3::OpenApiV3Spec) -> IntegrationTools {
    let mut tool_definitions: Vec<BionicToolDefinition> = vec![];
    let base_url = extract_base_url(&spec);

    // Process each operation in the OpenAPI spec
    for (_path, _method, operation) in spec.operations() {
        if let Some(operation_id) = &operation.operation_id {
            let function_name = operation_id.clone();
            let schema_key = format!("{}_form_model", function_name);

            // Extract parameters from operation definition
            let operation_params = extract_operation_parameters(operation);

            // Try to get schema-based parameters from components (backward compatibility)
            let schema_params = if let Some(components) = &spec.components {
                components
                    .schemas
                    .get(&schema_key)
                    .map(|schema| serde_json::to_value(schema).unwrap_or_default())
            } else {
                None
            };

            // Merge parameters (operation params take precedence)
            let parameters = merge_parameters(schema_params, operation_params);

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

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        tracing::info!(
            "Executing external integration tool {} with arguments: {}",
            self.name(),
            arguments
        );

        // Find operation details by operation_id
        let (path, method, operation) = self.find_operation_details()?;

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| json_error("Failed to parse arguments", e))?;

        // Substitute path parameters in the URL
        let path_with_params = substitute_path_parameters(&path, &args, operation)?;

        // Construct the final URL
        let url = format!("{}{}", self.base_url, path_with_params);
        tracing::debug!("Making request to URL: {} using method: {}", url, method);

        // Make the request based on the HTTP method
        // For GET requests, don't send JSON body if we have path parameters
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                // For GET requests, only send JSON body if there are no path parameters
                // or if there are additional non-path parameters
                if path.contains('{') && path_with_params != path {
                    // We substituted path parameters, so make a simple GET request
                    self
                        .client
                        .get(&url)
                        .send()
                        .await
                        .map_err(|e| json_error("Failed to make GET request", e))?
                } else {
                    // No path parameters, send as before
                    self.client
                        .get(&url)
                        .json(&args)
                        .send()
                        .await
                        .map_err(|e| json_error("Failed to make GET request", e))?
                }
            }
            "POST" => self
                .client
                .post(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| json_error("Failed to make POST request", e))?,
            "PUT" => self
                .client
                .put(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| json_error("Failed to make PUT request", e))?,
            "DELETE" => self
                .client
                .delete(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| json_error("Failed to make DELETE request", e))?,
            _ => {
                return Err(serde_json::json!({
                    "error": "Unsupported HTTP method",
                    "method": method
                }))
            }
        };

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(serde_json::json!({
                "error": "Request failed",
                "status": response.status().to_string()
            }));
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| json_error("Failed to read response", e))?;

        // Try to parse as JSON, fallback to text if it fails
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => Ok(json_value),
            Err(_) => Ok(serde_json::json!({
                "content": response_text,
                "content_type": "text"
            })),
        }
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

    fn create_uk_police_api_spec() -> oas3::OpenApiV3Spec {
        let spec_json = json!({
            "openapi": "3.0.3",
            "info": {
                "title": "UK Police Forces API",
                "version": "1.0.0",
                "description": "Provides a list of UK police forces and detailed data for each force."
            },
            "servers": [
                {
                    "url": "https://data.police.uk",
                    "description": "Production server"
                }
            ],
            "paths": {
                "/api/forces": {
                    "get": {
                        "summary": "List all police forces",
                        "operationId": "getForces",
                        "responses": {
                            "200": {
                                "description": "A list of police forces"
                            }
                        }
                    }
                },
                "/api/forces/{id}": {
                    "get": {
                        "summary": "Get details of a UK police force",
                        "operationId": "getPoliceForceDetails",
                        "parameters": [
                            {
                                "in": "path",
                                "name": "id",
                                "required": true,
                                "schema": {
                                    "type": "string"
                                },
                                "description": "The identifier of the police force (e.g. leicestershire)"
                            }
                        ],
                        "responses": {
                            "200": {
                                "description": "Police force details"
                            }
                        }
                    }
                }
            }
        });

        serde_json::from_value(spec_json).unwrap()
    }

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

    #[test]
    fn test_uk_police_api_parameter_extraction() {
        let spec = create_uk_police_api_spec();
        let integration_tools = create_tool_definitions_from_spec(spec);

        assert_eq!(integration_tools.tool_definitions.len(), 2);

        // Find getPoliceForceDetails tool
        let tool = integration_tools
            .tool_definitions
            .iter()
            .find(|t| t.function.name == "getPoliceForceDetails")
            .expect("getPoliceForceDetails tool should exist");

        // Verify parameters are correctly extracted
        assert!(tool.function.parameters.is_some());
        let params = tool.function.parameters.as_ref().unwrap();

        // Check JSON Schema structure
        assert_eq!(params["type"], "object");
        assert!(params["properties"]["id"].is_object());
        assert_eq!(params["properties"]["id"]["type"], "string");
        assert_eq!(
            params["properties"]["id"]["description"],
            "The identifier of the police force (e.g. leicestershire)"
        );
        assert_eq!(params["required"][0], "id");

        // Verify getForces has no parameters
        let get_forces_tool = integration_tools
            .tool_definitions
            .iter()
            .find(|t| t.function.name == "getForces")
            .expect("getForces tool should exist");
        assert!(get_forces_tool.function.parameters.is_none());
    }

    #[test]
    fn test_substitute_path_parameters() {
        let spec = create_uk_police_api_spec();

        // Find the getPoliceForceDetails operation
        let mut operation = None;
        for (_path, _method, op) in spec.operations() {
            if op.operation_id.as_ref() == Some(&"getPoliceForceDetails".to_string()) {
                operation = Some(op);
                break;
            }
        }

        let operation = operation.expect("Should find getPoliceForceDetails operation");
        let args = json!({"id": "leicestershire"});

        let result = substitute_path_parameters("/api/forces/{id}", &args, operation);
        assert_eq!(result.unwrap(), "/api/forces/leicestershire");
    }

    #[test]
    fn test_substitute_path_parameters_missing_required() {
        let spec = create_uk_police_api_spec();

        // Find the getPoliceForceDetails operation
        let mut operation = None;
        for (_path, _method, op) in spec.operations() {
            if op.operation_id.as_ref() == Some(&"getPoliceForceDetails".to_string()) {
                operation = Some(op);
                break;
            }
        }

        let operation = operation.expect("Should find getPoliceForceDetails operation");
        let args = json!({});

        let result = substitute_path_parameters("/api/forces/{id}", &args, operation);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Missing required path parameter: id"));
    }
}
