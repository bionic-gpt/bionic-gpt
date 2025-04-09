# Integrations

This crate provides a framework for integrating external tools and services with the application. It allows for the discovery and execution of functions provided by external services, following the OpenAI API tools format.

## Overview

The integrations crate provides:

1. A core `Integration` trait that all integrations must implement
2. An `IntegrationRegistry` to manage all registered integrations
3. A mechanism to dynamically load integrations from the database
4. An implementation of the `Integration` trait for MCP servers

## Usage

### Registering an Integration

Integrations are loaded from the database and registered with the `IntegrationRegistry`:

```rust
use integrations::{IntegrationRegistry, load_integrations};
use std::sync::Arc;

// Initialize the integration registry
let registry = match load_integrations(&pool).await {
    Ok(registry) => {
        tracing::info!("Integration registry initialized successfully");
        Some(Arc::new(registry))
    }
    Err(err) => {
        tracing::error!("Failed to initialize integration registry: {}", err);
        None
    }
};
```

### Discovering Tools

Tools can be discovered from all registered integrations:

```rust
let tools = registry.discover_all().await;
```

### Executing Functions

Functions can be executed on integrations:

```rust
let result = registry.execute("integration_name", "function_name", arguments).await?;
```

## Integration with LLM Proxy

The integrations crate is integrated with the LLM proxy to provide tools to language models:

```rust
// In function_tools.rs
pub async fn get_tools(registry: Option<&IntegrationRegistry>) -> Vec<Tool> {
    // ...
    if let Some(registry) = registry {
        match registry.discover_all().await {
            Ok(integration_tools) => tools.extend(integration_tools),
            Err(err) => {
                tracing::error!("Error discovering tools from integrations: {}", err);
            }
        }
    }
    // ...
}
```

## Creating a New Integration

To create a new integration, implement the `Integration` trait:

```rust
use crate::{Integration, IntegrationError};
use async_trait::async_trait;
use llm_proxy::{Tool, FunctionDefinition};

pub struct MyIntegration {
    id: i32,
    name: String,
    // ...
}

#[async_trait]
impl Integration for MyIntegration {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        "My Integration"
    }
    
    async fn discover(&self) -> Result<Vec<Tool>, IntegrationError> {
        // Return a list of tools provided by this integration
    }
    
    async fn execute(&self, function_name: &str, arguments: &str) -> Result<String, IntegrationError> {
        // Execute the function with the given arguments
    }
}
```

## MCP Server Integration

The MCP Server integration is a special type of integration that communicates with an MCP server to discover and execute functions. It expects the MCP server to provide the following endpoints:

- `/discover` - Returns a list of tools in the OpenAI API format
- `/execute` - Executes a function with the given arguments

The MCP Server integration prefixes each tool's function name with the integration name to avoid naming conflicts.