use crate::open_api_v3::{extract_base_url, open_api_to_definition};
use crate::tool::ToolInterface;
use async_trait::async_trait;
use openai_api::BionicToolDefinition;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

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

/// Create tools from integrations
pub async fn create_tools_from_integrations(
    integrations: Vec<db::queries::integrations::Integration>,
) -> Vec<Arc<dyn ToolInterface>> {
    let mut tools: Vec<Arc<dyn ToolInterface>> = Vec::new();

    for integration in integrations {
        if let Some(definition) = &integration.definition {
            let oas3 = oas3::from_json(definition.to_string());
            if let Ok(oas3) = oas3 {
                // Get base URL from the first server in the OpenAPI spec or use a default
                let base_url =
                    extract_base_url(&oas3).unwrap_or_else(|| "http://localhost".to_string());

                // Create tools for each operation in the OpenAPI spec
                let operations = open_api_to_definition(oas3);
                for operation in operations {
                    let tool = ExternalIntegrationTool::new(
                        operation.definition,
                        base_url.clone(),
                        operation.path,
                        operation.method,
                    );

                    tools.push(Arc::new(tool));
                }
            } else {
                tracing::error!(
                    "Failed to convert JSON in DB to oas3 for integration {}",
                    integration.id
                );
            }
        } else {
            tracing::error!("Integration {} doesn't have a definition", integration.id);
        }
    }

    tools
}
