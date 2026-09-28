use crate::{CredentialSet, SandboxFile, SandboxTool, WorkspaceDelta, WorkspaceSnapshot};
use async_trait::async_trait;
use serde_json::Value;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait Sandbox: Send + Sync {
    async fn run(&self, request: RunRequest) -> Result<RunResult, SandboxError>;
}

pub struct RunRequest {
    pub command: Command,
    pub skills: Vec<SandboxFile>,
    pub openapi_specs: Vec<OpenApiSpec>,
    pub credentials: CredentialSet,
    pub workspace: WorkspaceSnapshot,
    pub tools: Vec<Arc<dyn SandboxTool>>,
}

#[derive(Debug, Clone)]
pub enum Command {
    Shell {
        script: String,
        timeout: Duration,
    },
    Python {
        code: String,
        timeout: Duration,
    },
    ReadFile {
        path: String,
    },
    WriteFile {
        path: String,
        contents: Vec<u8>,
    },
    EditFile {
        path: String,
        find: Vec<u8>,
        replace: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenApiSpec {
    pub name: String,
    pub document: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u128,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    /// Raw result bytes for filesystem reads. Execution commands leave this unset.
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub execution: ExecutionResult,
    pub workspace_changes: WorkspaceDelta,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone)]
pub struct SandboxError(pub String);

impl fmt::Display for SandboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for SandboxError {}

pub(crate) fn error(error: impl ToString) -> SandboxError {
    SandboxError(error.to_string())
}
