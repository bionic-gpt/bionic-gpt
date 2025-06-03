use crate::tool::ToolInterface;
use async_trait::async_trait;
use chrono::{Local, Utc};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::{json, Value};

/// A tool that provides current time and date information
pub struct TimeDateTool;

#[async_trait]
impl ToolInterface for TimeDateTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_time_date_tool()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        // Since execute_time_date_tool is synchronous, we can just call it
        // It will be automatically wrapped in a future
        execute_time_date_tool(arguments)
    }
}

/// Returns a Tool definition for the time and date tool
pub fn get_time_date_tool() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "get_current_time_and_date".to_string(),
            description: Some(
                "Get the current time and date, optionally for a specific timezone".to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "timezone": {
                        "type": "string",
                        "description": "The timezone to get the time for (default: UTC)"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["iso", "human_readable"],
                        "description": "The format to return the time in"
                    }
                },
                "required": []
            })),
        },
    }
}

/// Execute the time and date tool with the given arguments
pub fn execute_time_date_tool(arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
    let args: Value =
        serde_json::from_str(arguments).map_err(|e| format!("Failed to parse arguments: {}", e))?;

    // Get current time in UTC
    let now_utc = Utc::now();

    // Default format is human readable
    let format = args["format"].as_str().unwrap_or("human_readable");

    // For now, we'll just support UTC and local time
    // A more comprehensive implementation would handle more timezones
    let timezone = args["timezone"].as_str().unwrap_or("utc");

    let time_str = if timezone.to_lowercase() == "local" {
        let now_local = Local::now();
        if format == "iso" {
            now_local.to_rfc3339()
        } else {
            now_local.format("%Y-%m-%d %H:%M:%S %Z").to_string()
        }
    } else {
        // Default to UTC
        if format == "iso" {
            now_utc.to_rfc3339()
        } else {
            now_utc.format("%Y-%m-%d %H:%M:%S UTC").to_string()
        }
    };

    Ok(json!({
        "current_time": time_str,
        "timestamp": now_utc.timestamp(),
        "timezone": timezone,
        "format": format
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_date_tool() {
        let tool = get_time_date_tool();
        assert_eq!(tool.function.name, "get_current_time_and_date");
    }

    #[test]
    fn test_execute_time_date_tool_valid() {
        let args = r#"{"timezone": "utc", "format": "human_readable"}"#;
        let result = execute_time_date_tool(args).unwrap();
        let parsed: Value = result;

        assert!(parsed["current_time"].is_string());
        assert!(parsed["timestamp"].is_number());
        assert_eq!(parsed["timezone"], "utc");
        assert_eq!(parsed["format"], "human_readable");
    }

    #[test]
    fn test_execute_time_date_tool_iso_format() {
        let args = r#"{"format": "iso"}"#;
        let result = execute_time_date_tool(args).unwrap();
        let parsed: Value = result;

        assert!(parsed["current_time"].is_string());
        assert_eq!(parsed["format"], "iso");
    }
}
