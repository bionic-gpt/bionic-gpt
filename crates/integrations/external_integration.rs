use crate::tool::ToolInterface;
use async_trait::async_trait;
use openai_api::BionicToolDefinition;
use reqwest::Client;
use serde_json::Value;

/// A tool that executes external integrations based on OpenAPI definitions
pub struct ExternalIntegrationTool {
    /// The tool definition
    definition: BionicToolDefinition,
    /// The base URL for the API
    base_url: String,
    /// The HTTP client
    client: Client,
    /// The path for this operation
    path: String,
    /// The HTTP method for this operation
    method: String,
}

impl ExternalIntegrationTool {
    pub fn new(
        definition: BionicToolDefinition,
        base_url: String,
        path: String,
        method: String,
    ) -> Self {
        Self {
            definition,
            base_url,
            client: Client::new(),
            path,
            method,
        }
    }
}

#[async_trait]
impl ToolInterface for ExternalIntegrationTool {
    fn get_tool(&self) -> BionicToolDefinition {
        self.definition.clone()
    }

    async fn execute(&self, arguments: &str) -> Result<String, String> {
        tracing::info!(
            "Executing external integration tool {} with arguments: {}",
            self.name(),
            arguments
        );

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Failed to parse arguments: {}", e))?;

        // Construct the URL
        let url = format!("{}{}", self.base_url, self.path);
        tracing::debug!("Making request to URL: {}", url);

        // Make the request based on the HTTP method
        let response = match self.method.to_uppercase().as_str() {
            "GET" => self
                .client
                .get(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make GET request: {}", e))?,
            "POST" => self
                .client
                .post(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make POST request: {}", e))?,
            "PUT" => self
                .client
                .put(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make PUT request: {}", e))?,
            "DELETE" => self
                .client
                .delete(&url)
                .json(&args)
                .send()
                .await
                .map_err(|e| format!("Failed to make DELETE request: {}", e))?,
            _ => return Err(format!("Unsupported HTTP method: {}", self.method)),
        };

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(format!("Request failed with status: {}", response.status()));
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        Ok(response_text)
    }
}
