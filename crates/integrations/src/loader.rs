//! Functions for loading integrations from the database.

use crate::integrations::date_time::DateTimeIntegration;
use crate::integrations::mcp_server::MCPServerIntegration;
use crate::{IntegrationError, IntegrationRegistry};
use db::{IntegrationType, Pool};
use std::sync::Arc;

/// Load all integrations from the database.
pub async fn load_integrations(pool: &Pool) -> Result<IntegrationRegistry, IntegrationError> {
    let registry = IntegrationRegistry::new();

    // Register built-in integrations
    registry.register(Arc::new(DateTimeIntegration::new()));

    let client = pool
        .get()
        .await
        .map_err(|e| IntegrationError::DatabaseError(e.to_string()))?;

    let integrations = db::queries::integrations::integrations()
        .bind(&client, &IntegrationType::MCP_Server)
        .all()
        .await
        .map_err(|e| IntegrationError::DatabaseError(e.to_string()))?;

    for integration in integrations {
        match integration.integration_type {
            IntegrationType::MCP_Server => {
                let mcp_integration = MCPServerIntegration::new(
                    integration.id,
                    integration.name,
                    integration.base_url,
                );
                registry.register(Arc::new(mcp_integration));
            }
        }
    }

    Ok(registry)
}
