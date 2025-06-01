use crate::open_api_v3::{extract_base_url, open_api_to_definition};
use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::Pool;
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

/// Create external integration tools from database integrations
pub async fn create_external_integration_tools(
    pool: &Pool,
    sub: String,
) -> Vec<Arc<dyn ToolInterface>> {
    let mut tools: Vec<Arc<dyn ToolInterface>> = Vec::new();

    // Get external integrations from the database
    tracing::debug!("Getting external integrations from database");

    let mut client = match pool.get().await {
        Ok(client) => {
            tracing::debug!("Successfully got database client");
            client
        }
        Err(e) => {
            tracing::error!("Failed to get database client: {}", e);
            return vec![];
        }
    };

    tracing::debug!("Creating transaction");
    let transaction = match client.transaction().await {
        Ok(transaction) => {
            tracing::debug!("Successfully created transaction");
            transaction
        }
        Err(e) => {
            tracing::error!("Failed to create transaction: {}", e);
            return vec![];
        }
    };

    // Set row-level security if sub is provided
    tracing::debug!("Setting row-level security for user: {}", sub);
    if let Err(e) = db::authz::set_row_level_security_user_id(&transaction, sub.clone()).await {
        tracing::error!("Failed to set row level security: {}", e);
        return vec![];
    }
    let external_integrations = match db::queries::integrations::integrations()
        .bind(&transaction)
        .all()
        .await
    {
        Ok(integrations) => {
            tracing::debug!("Found {} external integrations", integrations.len());
            integrations
        }
        Err(e) => {
            tracing::error!("Failed to get external integrations: {}", e);
            return vec![];
        }
    };

    for integration in external_integrations {
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
            tracing::error!("This integration doesn't have a definition");
        }
    }

    tracing::debug!("Created {} external integration tools", tools.len());
    tools
}
