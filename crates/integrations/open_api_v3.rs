use oas3;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};

pub fn open_api_to_definition(oas3: oas3::OpenApiV3Spec) -> Vec<BionicToolDefinition> {
    let mut definitions: Vec<BionicToolDefinition> = vec![];

    for (name, _method, operation) in oas3.operations() {
        definitions.push(BionicToolDefinition {
            r#type: "".to_string(),
            function: ChatCompletionFunctionDefinition {
                name,
                description: operation.description.clone(),
                parameters: None,
            },
        });
    }
    dbg!(&definitions);
    definitions
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
        let tool_definitions = open_api_to_definition(oas3_spec);

        // Currently, the function returns an empty vector, so we just check that
        // In a real implementation, we would add more assertions to verify the tool definitions
        assert_eq!(tool_definitions.len(), 0);
    }
}
