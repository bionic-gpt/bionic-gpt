//! Implementation of the MCP Server integration.

use crate::{models::Tool, Integration, IntegrationError};
use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;

/// Integration for MCP Servers.
pub struct MCPServerIntegration {
    #[allow(dead_code)]
    id: i32,
    name: String,
    base_url: String,
    client: Client,
}

impl MCPServerIntegration {
    /// Create a new MCP Server integration.
    pub fn new(id: i32, name: String, base_url: String) -> Self {
        Self {
            id,
            name,
            base_url,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Integration for MCPServerIntegration {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "MCP Server Integration"
    }

    async fn discover(&self) -> Result<Vec<Tool>, IntegrationError> {
        // Make a request to the MCP server to discover tools
        let url = format!("{}/discover", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(IntegrationError::CommunicationError)?;

        if !response.status().is_success() {
            return Err(IntegrationError::FunctionExecutionFailed(
                "discover".to_string(),
                format!("Server returned status: {}", response.status()),
            ));
        }

        let tools: Vec<Tool> = response
            .json()
            .await
            .map_err(IntegrationError::CommunicationError)?;

        // Prefix each tool's function name with the integration name
        let tools = tools
            .into_iter()
            .map(|mut tool| {
                let original_name = tool.function.name.clone();
                tool.function.name = format!("{}.{}", self.name, original_name);
                tool
            })
            .collect();

        Ok(tools)
    }

    async fn execute(
        &self,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, IntegrationError> {
        // Make a request to the MCP server to execute the function
        let url = format!("{}/execute", self.base_url);

        #[derive(Serialize)]
        struct ExecuteRequest<'a> {
            function: &'a str,
            arguments: &'a str,
        }

        let request = ExecuteRequest {
            function: function_name,
            arguments,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(IntegrationError::CommunicationError)?;

        if !response.status().is_success() {
            return Err(IntegrationError::FunctionExecutionFailed(
                function_name.to_string(),
                format!("Server returned status: {}", response.status()),
            ));
        }

        let result = response
            .text()
            .await
            .map_err(IntegrationError::CommunicationError)?;

        Ok(result)
    }
}
