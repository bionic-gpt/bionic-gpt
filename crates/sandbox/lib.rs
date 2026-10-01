//! Provider-neutral code execution.
//!
//! The public contract deliberately contains no Bionic database identifiers or
//! Bashkit lifecycle concepts. Callers supply filesystem and network capabilities.

mod bashkit;
mod contract;
mod filesystem;
mod network;

pub use bashkit::BashkitSandbox;
pub use contract::{Command, ExecutionResult, RunRequest, RunResult, Sandbox, SandboxError};
pub use filesystem::{
    DirectoryEntry, FileMetadata, FileType, FilesystemError, FilesystemErrorKind,
    SandboxFilesystem, WriteMode,
};
pub use network::{
    DenyNetwork, HttpHeader, HttpMethod, HttpRequest, HttpResponse, NetworkError, SandboxNetwork,
};

pub(crate) use contract::error;
