//! Implementation of the DateTime integration.

use crate::models::{FunctionDefinition, Tool};
use crate::{Integration, IntegrationError};
use async_trait::async_trait;
use chrono::{Datelike, TimeZone, Timelike, Utc};
use serde_json::{json, Value};

/// Integration for date and time services.
pub struct DateTimeIntegration {
    name: String,
}

impl DateTimeIntegration {
    /// Create a new DateTime integration.
    pub fn new() -> Self {
        Self {
            name: "date_time".to_string(),
        }
    }

    /// Execute the get_current_time function with the given arguments.
    fn execute_get_current_time(&self, arguments: &str) -> Result<String, IntegrationError> {
        let args: Value = serde_json::from_str(arguments).map_err(IntegrationError::JsonError)?;

        let timezone = args["timezone"].as_str().unwrap_or("UTC");

        // Get the current time
        let now = Utc::now();

        Ok(json!({
            "timezone": timezone,
            "iso_8601": now.to_rfc3339(),
            "unix_timestamp": now.timestamp(),
            "year": now.year(),
            "month": now.month(),
            "day": now.day(),
            "hour": now.hour(),
            "minute": now.minute(),
            "second": now.second(),
            "day_of_week": now.weekday().to_string(),
        })
        .to_string())
    }

    /// Execute the format_date function with the given arguments.
    fn execute_format_date(&self, arguments: &str) -> Result<String, IntegrationError> {
        let args: Value = serde_json::from_str(arguments).map_err(IntegrationError::JsonError)?;

        let timestamp = args["timestamp"].as_i64().ok_or_else(|| {
            IntegrationError::FunctionExecutionFailed(
                "format_date".to_string(),
                "Timestamp is required".to_string(),
            )
        })?;

        let format = args["format"].as_str().unwrap_or("%Y-%m-%d %H:%M:%S");

        // Parse the timestamp
        let dt = Utc.timestamp_opt(timestamp, 0).single().ok_or_else(|| {
            IntegrationError::FunctionExecutionFailed(
                "format_date".to_string(),
                "Invalid timestamp".to_string(),
            )
        })?;

        Ok(json!({
            "formatted_date": dt.format(format).to_string(),
            "iso_8601": dt.to_rfc3339(),
        })
        .to_string())
    }
}

#[async_trait]
impl Integration for DateTimeIntegration {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "Date and Time Integration"
    }

    async fn discover(&self) -> Result<Vec<Tool>, IntegrationError> {
        Ok(vec![
            Tool {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: format!("{}.get_current_time", self.name),
                    description: "Get the current date and time".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "timezone": {
                                "type": "string",
                                "description": "The timezone to use (defaults to UTC)"
                            }
                        },
                        "required": []
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: format!("{}.format_date", self.name),
                    description: "Format a Unix timestamp into a human-readable date".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "timestamp": {
                                "type": "integer",
                                "description": "The Unix timestamp to format"
                            },
                            "format": {
                                "type": "string",
                                "description": "The format string to use (defaults to %Y-%m-%d %H:%M:%S)"
                            }
                        },
                        "required": ["timestamp"]
                    }),
                },
            },
        ])
    }

    async fn execute(
        &self,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, IntegrationError> {
        match function_name {
            "get_current_time" => self.execute_get_current_time(arguments),
            "format_date" => self.execute_format_date(arguments),
            _ => Err(IntegrationError::FunctionNotFound(
                function_name.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_date_time_integration_discover() {
        let integration = DateTimeIntegration::new();

        let tools = integration.discover().await.unwrap();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].function.name, "date_time.get_current_time");
        assert_eq!(tools[1].function.name, "date_time.format_date");
    }

    #[tokio::test]
    async fn test_date_time_integration_execute_get_current_time() {
        let integration = DateTimeIntegration::new();

        let arguments = json!({
            "timezone": "UTC"
        })
        .to_string();

        let result = integration
            .execute("get_current_time", &arguments)
            .await
            .unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert!(result_json["iso_8601"].is_string());
        assert!(result_json["unix_timestamp"].is_i64());
        assert!(result_json["year"].is_i64());
    }

    #[tokio::test]
    async fn test_date_time_integration_execute_format_date() {
        let integration = DateTimeIntegration::new();

        let arguments = json!({
            "timestamp": 1609459200, // 2021-01-01 00:00:00 UTC
            "format": "%Y-%m-%d"
        })
        .to_string();

        let result = integration
            .execute("format_date", &arguments)
            .await
            .unwrap();
        let result_json: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["formatted_date"], "2021-01-01");
    }

    #[tokio::test]
    async fn test_date_time_integration_execute_unknown_function() {
        let integration = DateTimeIntegration::new();

        let arguments = json!({}).to_string();

        let result = integration.execute("unknown_function", &arguments).await;
        assert!(result.is_err());

        match result {
            Err(IntegrationError::FunctionNotFound(name)) => {
                assert_eq!(name, "unknown_function");
            }
            _ => panic!("Expected FunctionNotFound error"),
        }
    }
}
