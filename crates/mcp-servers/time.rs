use chrono::{TimeZone, Utc};
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    tool, tool_box, ServerHandler,
};
use schemars::JsonSchema;
use serde::Deserialize;
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

    let tool_box = TimeServer::tool_box();

    info!("{:?}", tool_box.list());
}
