//! BionicOpenAPI - Encapsulates OpenAPI specification operations
//!
//! This module provides a structured way to work with OpenAPI v3 specifications,
//! extracting tool definitions and handling parameter parsing.

use crate::tool::ToolInterface;
use oas3::{
    self,
    spec::{Operation, RequestBody},
};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::Value;
use std::sync::Arc;

/// Result of parsing an OpenAPI specification for tool creation
pub struct IntegrationTools {
    pub tool_definitions: Vec<BionicToolDefinition>,
    pub base_url: Option<String>,
}

/// A wrapper around an OpenAPI v3 specification that provides methods
/// for extracting tool definitions and handling OpenAPI operations
pub struct BionicOpenAPI {
    spec: oas3::OpenApiV3Spec,
}

impl BionicOpenAPI {
    /// Create a new BionicOpenAPI instance from an OpenAPI v3 specification
    pub fn new(spec: oas3::OpenApiV3Spec) -> Self {
        Self { spec }
    }

    /// Extract the base URL from the first server in the OpenAPI specification
    pub fn extract_base_url(&self) -> Option<String> {
        if !self.spec.servers.is_empty() {
            return Some(self.spec.servers[0].url.clone());
        }
        None
    }

    /// Create tool definitions from the OpenAPI specification
    pub fn create_tool_definitions(&self) -> IntegrationTools {
        let mut tool_definitions: Vec<BionicToolDefinition> = vec![];
        let base_url = self.extract_base_url();

        // Process each operation in the OpenAPI spec
        for (_path, _method, operation) in self.spec.operations() {
            if let Some(operation_id) = &operation.operation_id {
                let function_name = operation_id.clone();
                let schema_key = format!("{}_form_model", function_name);

                // Extract parameters from operation definition
                let operation_params = self.extract_operation_parameters(operation);

                // Extract request body schema
                let request_body_params = self.extract_request_body_schema(operation);

                // Try to get schema-based parameters from components (backward compatibility)
                let schema_params = if let Some(components) = &self.spec.components {
                    components
                        .schemas
                        .get(&schema_key)
                        .map(|schema| serde_json::to_value(schema).unwrap_or_default())
                } else {
                    None
                };

                // Merge parameters (operation params take precedence)
                let parameters =
                    self.merge_parameters(schema_params, operation_params, request_body_params);

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

    /// Extract parameters from an OpenAPI operation and convert to JSON Schema format
    fn extract_operation_parameters(&self, operation: &Operation) -> Option<Value> {
        let params = match operation.parameters(&self.spec) {
            Ok(p) => p,
            Err(_) => return None,
        };
        if params.is_empty() {
            return None;
        }

        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for param in params {
            let name = param.name.clone();

            // Determine the property definition from the parameter schema
            let mut property = if let Some(schema_or_ref) = &param.schema {
                match schema_or_ref.resolve(&self.spec) {
                    Ok(schema) => serde_json::to_value(&schema)
                        .ok()
                        .and_then(|v| v.as_object().cloned())
                        .unwrap_or_default(),
                    Err(_) => {
                        let mut map = serde_json::Map::new();
                        map.insert("type".to_string(), Value::String("string".to_string()));
                        map
                    }
                }
            } else {
                let mut map = serde_json::Map::new();
                map.insert("type".to_string(), Value::String("string".to_string()));
                map
            };

            if let Some(description) = &param.description {
                property.insert(
                    "description".to_string(),
                    Value::String(description.clone()),
                );
            }

            properties.insert(name.clone(), Value::Object(property));

            if param.required.unwrap_or(false) {
                required.push(name);
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

    /// Extract request body schema from an OpenAPI operation and convert to JSON Schema format
    fn extract_request_body_schema(&self, operation: &Operation) -> Option<Value> {
        let request_body_ref = operation.request_body.as_ref()?;
        let request_body = request_body_ref.resolve(&self.spec).ok()?;

        self.extract_schema_from_request_body_value(&request_body)
    }

    /// Extract schema from a request body JSON value
    fn extract_schema_from_request_body_value(&self, request_body: &RequestBody) -> Option<Value> {
        let json_content = request_body.content.get("application/json")?;
        let schema_ref = json_content.schema.as_ref()?;
        let schema = schema_ref.resolve(&self.spec).ok()?;
        serde_json::to_value(schema).ok()
    }

    /// Merge schema-based parameters with operation-based parameters and request body parameters
    /// Operation parameters take precedence over schema parameters
    fn merge_parameters(
        &self,
        schema_params: Option<Value>,
        operation_params: Option<Value>,
        request_body_params: Option<Value>,
    ) -> Option<Value> {
        // Start with schema params as base
        let mut result = schema_params.unwrap_or_else(|| {
            serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            })
        });

        // Ensure we have a proper object structure
        if !result.is_object() {
            result = serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            });
        }

        let result_obj = result.as_object_mut().unwrap();

        // Ensure properties and required arrays exist
        if !result_obj.contains_key("properties") {
            result_obj.insert(
                "properties".to_string(),
                Value::Object(serde_json::Map::new()),
            );
        }
        if !result_obj.contains_key("required") {
            result_obj.insert("required".to_string(), Value::Array(Vec::new()));
        }

        // Helper function to merge parameters into result
        let merge_params = |result_obj: &mut serde_json::Map<String, Value>, params: Value| {
            if let Value::Object(params_obj) = params {
                // Merge properties
                if let (
                    Some(Value::Object(ref mut result_props)),
                    Some(Value::Object(params_props)),
                ) = (
                    result_obj.get_mut("properties"),
                    params_obj.get("properties"),
                ) {
                    for (key, value) in params_props {
                        result_props.insert(key.clone(), value.clone());
                    }
                }

                // Merge required fields
                if let Some(Value::Array(params_required)) = params_obj.get("required") {
                    if let Some(Value::Array(ref mut result_required)) =
                        result_obj.get_mut("required")
                    {
                        for req in params_required {
                            if !result_required.contains(req) {
                                result_required.push(req.clone());
                            }
                        }
                    }
                }
            }
        };

        // Merge operation parameters (path/query params)
        if let Some(op_params) = operation_params {
            merge_params(result_obj, op_params);
        }

        // Merge request body parameters
        if let Some(rb_params) = request_body_params {
            merge_params(result_obj, rb_params);
        }

        // Return None if no properties were added
        if let Some(Value::Object(props)) = result_obj.get("properties") {
            if props.is_empty() {
                return None;
            }
        }

        Some(result)
    }
}

/// Extract the base URL from the first server in the OpenAPI specification
pub fn extract_base_url(oas3: &oas3::OpenApiV3Spec) -> Option<String> {
    let bionic_api = BionicOpenAPI::new(oas3.clone());
    bionic_api.extract_base_url()
}

/// Create tool definitions from an OpenAPI specification
pub fn create_tool_definitions_from_spec(spec: oas3::OpenApiV3Spec) -> IntegrationTools {
    let bionic_api = BionicOpenAPI::new(spec);
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
    fn test_create_tool_definitions_uses_operation_id() {
        let spec = create_test_openapi_spec();
        let bionic_api = BionicOpenAPI::new(spec);
        let integration_tools = bionic_api.create_tool_definitions();

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
        let bionic_api = BionicOpenAPI::new(spec);
        let base_url = bionic_api.extract_base_url();
        assert_eq!(base_url, Some("https://api.example.com".to_string()));
    }

    #[test]
    fn test_uk_police_api_parameter_extraction() {
        let spec = create_uk_police_api_spec();
        let bionic_api = BionicOpenAPI::new(spec);
        let integration_tools = bionic_api.create_tool_definitions();

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
    fn test_create_tool_definitions_uses_operation_id_wrapper() {
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
    fn test_extract_base_url_wrapper() {
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
