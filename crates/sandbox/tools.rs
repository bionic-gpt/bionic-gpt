use crate::Credential;
use async_trait::async_trait;
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolExposure {
    Python,
    Shell,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SandboxToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub exposure: ToolExposure,
    pub credential: Option<String>,
}

#[async_trait]
pub trait SandboxTool: Send + Sync {
    fn definition(&self) -> SandboxToolDefinition;

    async fn call(
        &self,
        arguments: Value,
        context: ToolCallContext<'_>,
    ) -> Result<Value, SandboxToolError>;
}

#[derive(Clone, Copy)]
pub struct ToolCallContext<'a> {
    pub files: &'a dyn ToolFiles,
    pub credential: Option<&'a Credential>,
}

#[async_trait]
pub trait ToolFiles: Send + Sync {
    async fn read(&self, path: &str) -> Result<Vec<u8>, SandboxToolError>;
    async fn write(&self, path: &str, contents: &[u8]) -> Result<(), SandboxToolError>;
}

#[derive(Debug, Clone)]
pub struct SandboxToolError(pub String);

impl fmt::Display for SandboxToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for SandboxToolError {}
