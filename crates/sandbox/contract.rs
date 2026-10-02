use crate::{SandboxFilesystem, SandboxNetwork};
use async_trait::async_trait;
use serde_json::{Map, Value};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait Sandbox: Send + Sync {
    async fn run(&self, request: RunRequest) -> Result<RunResult, SandboxError>;
}

pub struct RunRequest {
    pub command: Command,
    pub filesystem: Arc<dyn SandboxFilesystem>,
    pub network: Arc<dyn SandboxNetwork>,
    pub python_functions: Vec<Arc<dyn PythonFunction>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PythonCallArguments {
    pub positional: Vec<Value>,
    pub keyword: Map<String, Value>,
}

#[async_trait]
pub trait PythonFunction: Send + Sync {
    fn name(&self) -> &str;

    async fn call(&self, arguments: PythonCallArguments) -> Result<Value, PythonFunctionError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonFunctionError(pub String);

impl fmt::Display for PythonFunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for PythonFunctionError {}

#[derive(Debug, Clone)]
pub enum Command {
    Shell { script: String, timeout: Duration },
    Python { code: String, timeout: Duration },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u128,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub execution: ExecutionResult,
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
