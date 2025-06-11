//! OpenApiTool - HTTP request execution tool for OpenAPI operations
//!
//! This module provides the OpenApiTool struct that executes HTTP requests
//! based on OpenAPI operation definitions.

use crate::tool::ToolInterface;
use async_trait::async_trait;
use oas3::{self, spec::Operation};
use openai_api::BionicToolDefinition;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;

/// A tool that executes external integrations based on OpenAPI definitions
pub struct OpenApiTool {
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

impl OpenApiTool {
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
impl ToolInterface for OpenApiTool {
    fn get_tool(&self) -> BionicToolDefinition {
        self.definition.clone()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        tracing::info!(
            "Executing OpenAPI tool {} with arguments: {}",
            self.name(),
            arguments
        );

        // Find operation details by operation_id
        let (path, method, operation) = self
            .find_operation_details()
            .map_err(|e| crate::json_error("Operation not found", e))?;

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| crate::json_error("Failed to parse arguments", e))?;

        // Separate path/query parameters from request body parameters
        let (path_query_params, request_body_params) = separate_parameters(&args, operation)
            .map_err(|e| crate::json_error("Failed to separate parameters", e))?;

        tracing::debug!(
            "Separated parameters - Path/Query: {}, Request Body: {}",
            serde_json::to_string(&path_query_params).unwrap_or_default(),
            serde_json::to_string(&request_body_params).unwrap_or_default()
        );

        // Substitute path parameters in the URL using only path/query params
        let path_with_params = substitute_path_parameters(&path, &path_query_params, operation)
            .map_err(|e| crate::json_error("Failed to substitute path parameters", e))?;

        // Construct the final URL
        let url = format!("{}{}", self.base_url, path_with_params);
        tracing::debug!("Making request to URL: {} using method: {}", url, method);

        // Determine if we should send a request body
        let body_obj = request_body_params.as_object();
        let has_request_body = body_obj.is_some_and(|obj| !obj.is_empty());
        if body_obj.is_none() {
            return Err(serde_json::json!({
                "error": "Malformed request body arguments"
            }));
        }

        // Make the request based on the HTTP method
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                // GET requests typically don't have request bodies
                // Send query parameters in URL if any (future enhancement)
                self.client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| crate::json_error("Failed to make GET request", e))?
            }
            "POST" => {
                let mut request = self.client.post(&url);
                if has_request_body {
                    request = request.json(&request_body_params);
                }
                request
                    .send()
                    .await
                    .map_err(|e| crate::json_error("Failed to make POST request", e))?
            }
            "PUT" => {
                let mut request = self.client.put(&url);
                if has_request_body {
                    request = request.json(&request_body_params);
                }
                request
                    .send()
                    .await
                    .map_err(|e| crate::json_error("Failed to make PUT request", e))?
            }
            "DELETE" => {
                let mut request = self.client.delete(&url);
                if has_request_body {
                    request = request.json(&request_body_params);
                }
                request
                    .send()
                    .await
                    .map_err(|e| crate::json_error("Failed to make DELETE request", e))?
            }
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
            .map_err(|e| crate::json_error("Failed to read response", e))?;

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

/// Separate path/query parameters from request body parameters
fn separate_parameters(args: &Value, operation: &Operation) -> Result<(Value, Value), String> {
    let mut path_query_params = serde_json::Map::new();
    let mut request_body_params = serde_json::Map::new();

    // Get all arguments as an object
    let args_obj = args.as_object().ok_or("Arguments must be a JSON object")?;

    // Collect path and query parameter names from the operation
    let mut path_query_param_names = HashSet::new();

    for param in &operation.parameters {
        if let Ok(param_value) = serde_json::to_value(param) {
            if let Some(param_obj) = param_value.as_object() {
                if let Some(location) = param_obj.get("in").and_then(|l| l.as_str()) {
                    if location == "path" || location == "query" {
                        if let Some(param_name) = param_obj.get("name").and_then(|n| n.as_str()) {
                            path_query_param_names.insert(param_name.to_string());
                        }
                    }
                }
            }
        }
    }

    // Separate the arguments based on parameter type
    for (key, value) in args_obj {
        if path_query_param_names.contains(key) {
            path_query_params.insert(key.clone(), value.clone());
        } else {
            request_body_params.insert(key.clone(), value.clone());
        }
    }

    Ok((
        Value::Object(path_query_params),
        Value::Object(request_body_params),
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use openai_api::ChatCompletionFunctionDefinition;
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

    #[test]
    fn test_openapi_tool_find_operation_details() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "getUsers".to_string(),
                description: Some("Get all users".to_string()),
                parameters: None,
            },
        };

        let tool = OpenApiTool::new(
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
    fn test_openapi_tool_operation_not_found() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "nonExistentOperation".to_string(),
                description: Some("Non-existent operation".to_string()),
                parameters: None,
            },
        };

        let tool = OpenApiTool::new(
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

        let tool = OpenApiTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "createUser".to_string(),
        );

        assert_eq!(tool.name(), "createUser");
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
