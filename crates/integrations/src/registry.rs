//! Registry for managing integrations.

use crate::integrations::date_time::DateTimeIntegration;
use crate::integrations::mcp_server::MCPServerIntegration;
use crate::{models::Tool, Integration, IntegrationError};
use db::{IntegrationType, Pool};
use std::sync::Arc;

/// Registry for managing all registered integrations.
pub struct IntegrationRegistry {
    pool: Pool,
}

impl IntegrationRegistry {
    /// Create a new registry with a database pool.
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Load all integrations from the database.
    async fn load_integrations(&self) -> Result<Vec<Arc<dyn Integration>>, IntegrationError> {
        let mut integrations: Vec<Arc<dyn Integration>> = Vec::new();

        // Add built-in integrations
        integrations.push(Arc::new(DateTimeIntegration::new()));

        // Load integrations from the database
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| IntegrationError::DatabaseError(e.to_string()))?;

        let db_integrations = db::queries::integrations::integrations()
            .bind(&client, &IntegrationType::MCP_Server)
            .all()
            .await
            .map_err(|e| IntegrationError::DatabaseError(e.to_string()))?;

        for integration in db_integrations {
            match integration.integration_type {
                IntegrationType::MCP_Server => {
                    let mcp_integration = MCPServerIntegration::new(
                        integration.id,
                        integration.name,
                        integration.base_url,
                    );
                    integrations.push(Arc::new(mcp_integration));
                }
            }
        }

        Ok(integrations)
    }

    /// Discover all tools from all registered integrations.
    pub async fn discover_all(&self) -> Vec<Tool> {
        let mut tools = Vec::new();

        match self.load_integrations().await {
            Ok(integrations) => {
                for integration in integrations {
                    match integration.discover().await {
                        Ok(mut integration_tools) => tools.append(&mut integration_tools),
                        Err(err) => {
                            tracing::error!(
                                "Error discovering tools for integration {}: {}",
                                integration.name(),
                                err
                            );
                        }
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error loading integrations: {}", err);
            }
        }

        tools
    }

    /// Execute a function on an integration.
    pub async fn execute(
        &self,
        integration_name: &str,
        function_name: &str,
        arguments: &str,
    ) -> Result<String, IntegrationError> {
        let integrations = self.load_integrations().await?;

        for integration in integrations {
            if integration.name() == integration_name {
                return integration.execute(function_name, arguments).await;
            }
        }

        Err(IntegrationError::IntegrationNotFound(
            integration_name.to_string(),
        ))
    }
}
