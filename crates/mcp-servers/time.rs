use chrono::{TimeZone, Utc};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    tool, tool_box, ServerHandler,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

// Define request structs for our tools
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FormatTimeRequest {
    #[schemars(description = "The timestamp to format (Unix timestamp in seconds)")]
    pub timestamp: i64,
    #[schemars(description = "The format string (e.g., '%Y-%m-%d %H:%M:%S')")]
    pub format: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeDifferenceRequest {
    #[schemars(description = "The first timestamp (Unix timestamp in seconds)")]
    pub timestamp1: i64,
    #[schemars(description = "The second timestamp (Unix timestamp in seconds)")]
    pub timestamp2: i64,
}

#[derive(Debug, Clone)]
pub struct TimeServer;

impl TimeServer {
    #[tool(description = "Get the current time")]
    fn current_time(&self) -> String {
        let now = Utc::now();
        format!("Current UTC time: {}", now.format("%Y-%m-%d %H:%M:%S"))
    }

    #[tool(description = "Format a timestamp according to a specified format")]
    fn format_time(
        &self,
        #[tool(aggr)] FormatTimeRequest { timestamp, format }: FormatTimeRequest,
    ) -> String {
        match Utc.timestamp_opt(timestamp, 0) {
            chrono::LocalResult::Single(dt) => {
                format!("Formatted time: {}", dt.format(&format))
            }
            _ => "Invalid timestamp".to_string(),
        }
    }

    #[tool(description = "Calculate the difference between two timestamps")]
    fn time_difference(
        &self,
        #[tool(aggr)] TimeDifferenceRequest {
            timestamp1,
            timestamp2,
        }: TimeDifferenceRequest,
    ) -> String {
        match (
            Utc.timestamp_opt(timestamp1, 0),
            Utc.timestamp_opt(timestamp2, 0),
        ) {
            (chrono::LocalResult::Single(dt1), chrono::LocalResult::Single(dt2)) => {
                let duration = if dt1 > dt2 {
                    dt1.signed_duration_since(dt2)
                } else {
                    dt2.signed_duration_since(dt1)
                };

                let days = duration.num_days();
                let hours = duration.num_hours() % 24;
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;

                format!(
                    "Time difference: {} days, {} hours, {} minutes, {} seconds",
                    days, hours, minutes, seconds
                )
            }
            _ => "Invalid timestamp(s)".to_string(),
        }
    }

    /// Returns a list of tools in BionicToolDefinition format
    pub fn get_bionic_tools(&self) -> Vec<BionicToolDefinition> {
        let tool_box = Self::tool_box();
        let tools = tool_box.list();

        tools
            .iter()
            .map(|tool_info| BionicToolDefinition {
                r#type: "function".to_string(),
                function: ChatCompletionFunctionDefinition {
                    name: tool_info.name.to_string(),
                    description: Some(tool_info.description.to_string()),
                    parameters: Some(
                        serde_json::to_value(tool_info.input_schema.clone())
                            .unwrap_or(serde_json::Value::Null),
                    ),
                },
            })
            .collect()
    }

    /// Executes a tool by name with the given arguments
    pub fn execute_tool(&self, name: &str, arguments: &str) -> Result<String, String> {
        let tool_box = Self::tool_box();

        // Find the tool by name
        let _tool = tool_box
            .map
            .get(name)
            .ok_or_else(|| format!("Unknown tool: {}", name))?;

        // Parse the arguments as JSON
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Failed to parse arguments: {}", e))?;

        // Execute the tool
        match name {
            "current_time" => Ok(self.current_time()),
            "format_time" => {
                let timestamp = args["timestamp"]
                    .as_i64()
                    .ok_or_else(|| "Missing or invalid timestamp".to_string())?;

                let format = args["format"]
                    .as_str()
                    .ok_or_else(|| "Missing or invalid format".to_string())?
                    .to_string();

                let request = FormatTimeRequest { timestamp, format };
                Ok(self.format_time(request))
            }
            "time_difference" => {
                let timestamp1 = args["timestamp1"]
                    .as_i64()
                    .ok_or_else(|| "Missing or invalid timestamp1".to_string())?;

                let timestamp2 = args["timestamp2"]
                    .as_i64()
                    .ok_or_else(|| "Missing or invalid timestamp2".to_string())?;

                let request = TimeDifferenceRequest {
                    timestamp1,
                    timestamp2,
                };
                Ok(self.time_difference(request))
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }

    tool_box!(TimeServer {
        current_time,
        format_time,
        time_difference
    });
}

impl ServerHandler for TimeServer {
    // We'll need the blow for a remote MCP server
    //tool_box!(@derive);

    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A server that provides time-related functionality".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let time_server = TimeServer;

    // Get and print the list of tools in BionicToolDefinition format
    let bionic_tools = time_server.get_bionic_tools();
    info!("Bionic Tools: {:#?}", bionic_tools);

    // Execute a tool and print the result
    let args = r#"{"timestamp": 1620000000, "format": "%Y-%m-%d %H:%M:%S"}"#;
    match time_server.execute_tool("format_time", args) {
        Ok(result) => info!("Tool execution result: {}", result),
        Err(err) => info!("Tool execution error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_bionic_tools() {
        let server = TimeServer;
        let tools = server.get_bionic_tools();

        // Check that we have the expected number of tools
        assert_eq!(tools.len(), 3);

        // Check that the tools have the expected names
        let tool_names: Vec<String> = tools.iter().map(|t| t.function.name.clone()).collect();

        assert!(tool_names.contains(&"current_time".to_string()));
        assert!(tool_names.contains(&"format_time".to_string()));
        assert!(tool_names.contains(&"time_difference".to_string()));

        // Check that all tools have descriptions
        for tool in &tools {
            assert!(tool.function.description.is_some());
        }
    }

    #[test]
    fn test_execute_tool_current_time() {
        let server = TimeServer;
        let result = server.execute_tool("current_time", "{}");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Current UTC time:"));
    }

    #[test]
    fn test_execute_tool_format_time() {
        let server = TimeServer;
        let args = json!({
            "timestamp": 1620000000,
            "format": "%Y-%m-%d %H:%M:%S"
        })
        .to_string();

        let result = server.execute_tool("format_time", &args);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Formatted time:"));
        assert!(output.contains("2021-05-03"));
    }

    #[test]
    fn test_execute_tool_time_difference() {
        let server = TimeServer;
        let args = json!({
            "timestamp1": 1620000000,
            "timestamp2": 1620086400
        })
        .to_string();

        let result = server.execute_tool("time_difference", &args);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Time difference:"));
        assert!(output.contains("1 days"));
    }

    #[test]
    fn test_execute_tool_unknown_tool() {
        let server = TimeServer;
        let result = server.execute_tool("unknown_tool", "{}");

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unknown tool:"));
    }

    #[test]
    fn test_execute_tool_invalid_arguments() {
        let server = TimeServer;
        let result = server.execute_tool("format_time", "invalid json");

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Failed to parse arguments:"));
    }

    #[test]
    fn test_execute_tool_missing_required_arguments() {
        let server = TimeServer;
        let result = server.execute_tool("format_time", "{}");

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Missing or invalid"));
    }
}
