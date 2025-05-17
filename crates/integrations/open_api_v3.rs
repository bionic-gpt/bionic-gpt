use oas3;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};

/// Represents an operation from an OpenAPI specification with its path and method
pub struct OpenApiOperation {
    pub definition: BionicToolDefinition,
    pub path: String,
    pub method: String,
}

/// Convert an OpenAPI specification to a list of tool definitions with path and method information
pub fn open_api_to_definition(oas3: oas3::OpenApiV3Spec) -> Vec<OpenApiOperation> {
    let mut operations: Vec<OpenApiOperation> = vec![];

    for (path, method, operation) in oas3.operations() {
        if let Some(_operation_id) = &operation.operation_id {
            let mut parameters = None;
            let function_name = path.replace('/', "");
            let schema_key = format!("{}_form_model", function_name);

            if let Some(components) = &oas3.components {
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
                    description: operation.description.clone(),
                    parameters,
                },
            };

            operations.push(OpenApiOperation {
                definition,
                path: path.to_string(),
                method: method.to_string(),
            });
        }
    }

    operations
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

        dbg!(&tool_definitions);

        // Verify the tool definitions
        assert_eq!(tool_definitions.len(), 1);
        assert_eq!(tool_definitions[0].function.name, "get_current_time");
    }
}
