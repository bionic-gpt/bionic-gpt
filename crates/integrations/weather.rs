use crate::{tool::ToolInterface, BionicToolDefinition};
use openai::chat::ChatCompletionFunctionDefinition;
use serde_json::{json, Value};

/// A tool that provides weather information
pub struct WeatherTool;

impl ToolInterface for WeatherTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_weather_tool()
    }

    fn execute(&self, arguments: &str) -> Result<String, String> {
        execute_weather_function(arguments)
    }
}

/// Returns a Tool definition for the weather function
pub fn get_weather_tool() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "get_weather".to_string(),
            description: Some("Get the current weather in a given location".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "The temperature unit to use"
                    }
                },
                "required": ["location"]
            })),
        },
    }
}

/// Execute the weather function with the given arguments
/// This is a mock implementation that returns fixed data
pub fn execute_weather_function(arguments: &str) -> Result<String, String> {
    let args: Value =
        serde_json::from_str(arguments).map_err(|e| format!("Failed to parse arguments: {}", e))?;

    let location = args["location"]
        .as_str()
        .ok_or_else(|| "Location is required".to_string())?;

    let unit = args["unit"].as_str().unwrap_or("celsius");

    // Mock implementation - return fixed data
    let temp = if unit == "celsius" { 22 } else { 72 };
    let condition = "sunny";

    Ok(json!({
        "location": location,
        "temperature": temp,
        "unit": unit,
        "condition": condition,
        "forecast": ["sunny", "partly cloudy", "sunny"]
    })
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weather_tool() {
        let tool = get_weather_tool();
        assert_eq!(tool.function.name, "get_weather");
    }

    #[test]
    fn test_execute_weather_function_valid() {
        let args = r#"{"location": "San Francisco, CA", "unit": "celsius"}"#;
        let result = execute_weather_function(args).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["location"], "San Francisco, CA");
        assert_eq!(parsed["temperature"], 22);
        assert_eq!(parsed["unit"], "celsius");
    }

    #[test]
    fn test_execute_weather_function_missing_location() {
        let args = r#"{"unit": "celsius"}"#;
        let result = execute_weather_function(args);
        assert!(result.is_err());
    }
}
