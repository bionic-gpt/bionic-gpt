//! Provider-neutral code execution and workspace synchronization.
//!
//! The public contract deliberately contains no Bionic database identifiers or
//! Bashkit lifecycle concepts. Callers supply a complete execution snapshot and
//! authorization-scoped callback tools for each run.

mod bashkit;
mod contract;
mod credentials;
mod tools;
mod workspace;

pub use bashkit::BashkitSandbox;
pub use contract::{
    Artifact, Command, ExecutionResult, OpenApiSpec, RunRequest, RunResult, Sandbox, SandboxError,
};
pub use credentials::{Credential, CredentialSet};
pub use tools::{
    SandboxTool, SandboxToolDefinition, SandboxToolError, ToolCallContext, ToolExposure, ToolFiles,
};
pub use workspace::{
    SandboxFile, WorkspaceDelta, WorkspaceSnapshot, HOME_DIR, OUTPUT_DIR, WORK_DIR,
};

pub(crate) use contract::error;
pub(crate) use workspace::{diff_workspace, mutable_files};
