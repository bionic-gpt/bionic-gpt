use oas3;
use oas3::spec::{ObjectOrReference, Parameter, ParameterIn, PathItem};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};

/// Represents an operation from an OpenAPI specification with its path and method
pub struct OpenApiOperation {
    pub definition: BionicToolDefinition,
    pub path: String,
    pub method: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub location: String,
    pub description: Option<String>,
    pub required: bool,
    pub source: ParameterSource,
}

#[derive(Debug, Clone)]
pub enum ParameterSource {
    PathItem,
    Operation(String),
}

fn resolve_parameter(param_ref: &ObjectOrReference<Parameter>) -> Option<Parameter> {
    match param_ref {
        ObjectOrReference::Object(param) => Some(param.clone()),
        ObjectOrReference::Ref { .. } => {
            // For now, skip references as they require spec resolution
            // Could be enhanced later to resolve references
            None
        }
    }
}

fn parameter_location_to_string(location: &ParameterIn) -> String {
    match location {
        ParameterIn::Path => "path".to_string(),
        ParameterIn::Query => "query".to_string(),
        ParameterIn::Header => "header".to_string(),
        ParameterIn::Cookie => "cookie".to_string(),
    }
}

fn extract_parameters_info(_path: &str, item: &PathItem) -> Vec<ParameterInfo> {
    let mut parameters = Vec::new();

    // Extract PathItem-level parameters
    for param_ref in &item.parameters {
        if let Some(param) = resolve_parameter(param_ref) {
            parameters.push(ParameterInfo {
                name: param.name,
                location: parameter_location_to_string(&param.location),
                description: param.description,
                required: param.required.unwrap_or(false),
                source: ParameterSource::PathItem,
            });
        }
    }

    // Extract Operation-level parameters for each HTTP method
    let operations = [
        ("GET", &item.get),
        ("PUT", &item.put),
        ("POST", &item.post),
        ("DELETE", &item.delete),
        ("OPTIONS", &item.options),
        ("HEAD", &item.head),
        ("PATCH", &item.patch),
        ("TRACE", &item.trace),
    ];

    for (method, operation_opt) in operations {
        if let Some(operation) = operation_opt {
            for param_ref in &operation.parameters {
                if let Some(param) = resolve_parameter(param_ref) {
                    parameters.push(ParameterInfo {
                        name: param.name,
                        location: parameter_location_to_string(&param.location),
                        description: param.description,
                        required: param.required.unwrap_or(false),
                        source: ParameterSource::Operation(method.to_string()),
                    });
                }
            }
        }
    }

    parameters
}

/// Convert an OpenAPI specification to a list of tool definitions with path and method information
pub fn open_api_to_definition(oas3: oas3::OpenApiV3Spec) -> Vec<OpenApiOperation> {
    let mut operations: Vec<OpenApiOperation> = vec![];

    // First, we need to get the paths to extract parameter info
    if let Some(paths) = &oas3.paths {
        for (path_str, path_item) in paths {
            // Extract parameter info for this path
            let path_parameters = extract_parameters_info(path_str, path_item);

            // Process each operation in this path
            for (path, method, operation) in oas3.operations() {
                if path != *path_str {
                    continue; // Only process operations for the current path
                }

                if let Some(_operation_id) = &operation.operation_id {
                    let mut parameters = None;
                    let function_name = path.replace('/', "");
                    let schema_key = format!("{}_form_model", function_name);

                    // Try to get schema-based parameters first (existing behavior)
                    if let Some(components) = &oas3.components {
                        let schema = components.schemas.get(&schema_key);
                        if let Some(schema) = schema {
                            let params = serde_json::to_value(schema).unwrap_or_default();
                            parameters = Some(params);
                        }
                    }

                    // Filter parameters for this specific operation
                    let operation_parameters: Vec<ParameterInfo> = path_parameters
                        .iter()
                        .filter(|param| match &param.source {
                            ParameterSource::PathItem => true,
                            ParameterSource::Operation(op_method) => {
                                op_method.to_uppercase() == method.to_string().to_uppercase()
                            }
                        })
                        .cloned()
                        .collect();

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

                    operations.push(OpenApiOperation {
                        definition,
                        path: path.to_string(),
                        method: method.to_string(),
                        summary: operation.summary.clone(),
                        description: operation.description.clone(),
                        operation_id: operation.operation_id.clone(),
                        parameters: operation_parameters,
                    });
                }
            }
        }
    }

    operations
}

/// Extract the base URL from the first server in the OpenAPI specification
pub fn extract_base_url(oas3: &oas3::OpenApiV3Spec) -> Option<String> {
    if !oas3.servers.is_empty() {
        return Some(oas3.servers[0].url.clone());
    }
    None
}

/// Legacy function for backward compatibility
pub fn open_api_to_definition_legacy(oas3: oas3::OpenApiV3Spec) -> Vec<BionicToolDefinition> {
    open_api_to_definition(oas3)
        .into_iter()
        .map(|op| op.definition)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_open_api_to_definition() {
        // Create a simplified OpenAPI spec for testing
        let json = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "mcp-time",
                "description": "mcp-time MCP Server",
                "version": "1.0.0"
            },
            "paths": {
                "/get_current_time": {
                    "post": {
                        "summary": "Get Current Time",
                        "description": "Get current time in a specific timezone",
                        "operationId": "tool_get_current_time_post"
                    }
                }
            }
        });

        // Convert to string and parse as oas3::OpenApiV3Spec
        let json_str = serde_json::to_string(&json).expect("Failed to convert JSON to string");
        let oas3_spec: oas3::OpenApiV3Spec =
            serde_json::from_str(&json_str).expect("Failed to parse OpenAPI JSON");

        // Call the function being tested
        let operations = open_api_to_definition(oas3_spec);

        // Verify the operations
        assert_eq!(operations.len(), 1);

        let operation = &operations[0];
        assert_eq!(operation.path, "/get_current_time");
        assert_eq!(operation.method, "POST");
        assert_eq!(operation.definition.function.name, "get_current_time");
        assert_eq!(
            operation.definition.function.description,
            Some("Get current time in a specific timezone".to_string())
        );
    }

    #[test]
    fn test_open_api_to_definition_legacy() {
        // Create a simplified OpenAPI spec for testing
        let json = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "mcp-time",
                "description": "mcp-time MCP Server",
                "version": "1.0.0"
            },
            "paths": {
                "/get_current_time": {
                    "post": {
                        "summary": "Get Current Time",
                        "description": "Get current time in a specific timezone",
                        "operationId": "tool_get_current_time_post",
                        "requestBody":{
                            "content":{
                                "application/json":{
                                    "schema":{"$ref":"#/components/schemas/get_current_time_form_model"}
                                }
                            },"required":true
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "get_current_time_form_model": {
                        "type": "object",
                        "title": "get_current_time_form_model",
                        "properties": {
                            "timezone": {
                                "type": "string",
                                "title": "Timezone",
                                "description": "IANA timezone name (e.g., 'America/New_York', 'Europe/London'). Use 'America/New_York' as local timezone if no timezone provided by the user."
                            }
                        },
                        "required": [
                            "timezone"
                        ]
                    }
                }
            }
        });

        // Convert to string and parse as oas3::OpenApiV3Spec
        let json_str = serde_json::to_string(&json).expect("Failed to convert JSON to string");
        let oas3_spec: oas3::OpenApiV3Spec =
            serde_json::from_str(&json_str).expect("Failed to parse OpenAPI JSON");
        let tool_definitions = open_api_to_definition_legacy(oas3_spec);

        // Verify the tool definitions
        assert_eq!(tool_definitions.len(), 1);
        assert_eq!(tool_definitions[0].function.name, "get_current_time");

        assert!(tool_definitions[0].function.parameters.is_some());
    }

    #[test]
    fn test_open_api_to_definition_summary_fallback() {
        // Create a simplified OpenAPI spec with only summary (no description)
        let json = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Blockchain API",
                "description": "Blockchain API Server",
                "version": "1.0.0"
            },
            "paths": {
                "/ticker": {
                    "get": {
                        "summary": "Get Bitcoin prices by currency",
                        "operationId": "getTicker"
                    }
                }
            }
        });

        // Convert to string and parse as oas3::OpenApiV3Spec
        let json_str = serde_json::to_string(&json).expect("Failed to convert JSON to string");
        let oas3_spec: oas3::OpenApiV3Spec =
            serde_json::from_str(&json_str).expect("Failed to parse OpenAPI JSON");

        // Call the function being tested
        let operations = open_api_to_definition(oas3_spec);

        // Verify the operations
        assert_eq!(operations.len(), 1);

        let operation = &operations[0];
        assert_eq!(operation.path, "/ticker");
        assert_eq!(operation.method, "GET");
        assert_eq!(operation.definition.function.name, "ticker");

        // Verify that the summary is used as fallback for description
        assert_eq!(
            operation.definition.function.description,
            Some("Get Bitcoin prices by currency".to_string())
        );
    }

    #[test]
    fn test_extract_base_url() {
        // Test with servers array
        let json_with_servers = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "servers": [
                {
                    "url": "https://api.example.com/v1"
                },
                {
                    "url": "https://staging.example.com/v1"
                }
            ],
            "paths": {}
        });

        let json_str = serde_json::to_string(&json_with_servers).unwrap();
        let oas3_spec: oas3::OpenApiV3Spec = serde_json::from_str(&json_str).unwrap();

        let base_url = extract_base_url(&oas3_spec);
        assert_eq!(base_url, Some("https://api.example.com/v1".to_string()));

        // Test without servers array
        let json_without_servers = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "paths": {}
        });

        let json_str = serde_json::to_string(&json_without_servers).unwrap();
        let oas3_spec: oas3::OpenApiV3Spec = serde_json::from_str(&json_str).unwrap();

        let base_url = extract_base_url(&oas3_spec);
        assert_eq!(base_url, None);

        // Test with empty servers array
        let json_empty_servers = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "servers": [],
            "paths": {}
        });

        let json_str = serde_json::to_string(&json_empty_servers).unwrap();
        let oas3_spec: oas3::OpenApiV3Spec = serde_json::from_str(&json_str).unwrap();

        let base_url = extract_base_url(&oas3_spec);
        assert_eq!(base_url, None);
    }
}
