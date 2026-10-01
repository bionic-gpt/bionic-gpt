mod execution;
mod filesystem;
mod network;

use crate::{RunRequest, RunResult, Sandbox, SandboxError};
use async_trait::async_trait;
use bashkit::{Bash, ExecutionLimits, FileSystem, NetworkAllowlist, PythonLimits, SqliteLimits};
use execution::{command_timeout, MAX_COMMANDS, MAX_STDERR_BYTES, MAX_STDOUT_BYTES};
use filesystem::FilesystemAdapter;
use network::NetworkAdapter;
use std::sync::Arc;

const HOME_DIR: &str = "/home/user";

#[derive(Default)]
pub struct BashkitSandbox;

#[async_trait]
impl Sandbox for BashkitSandbox {
    async fn run(&self, request: RunRequest) -> Result<RunResult, SandboxError> {
        let fs: Arc<dyn FileSystem> = Arc::new(FilesystemAdapter(request.filesystem));
        let timeout = command_timeout(&request.command);
        let mut bash = Bash::builder()
            .fs(fs.clone())
            .username("user")
            .hostname("sandbox")
            .cwd(HOME_DIR)
            .env("BASHKIT_ALLOW_INPROCESS_PYTHON", "1")
            .env("BASHKIT_ALLOW_INPROCESS_SQLITE", "1")
            .limits(
                ExecutionLimits::new()
                    .timeout(timeout)
                    .max_commands(MAX_COMMANDS)
                    .max_stdout_bytes(MAX_STDOUT_BYTES)
                    .max_stderr_bytes(MAX_STDERR_BYTES),
            )
            .python_with_limits(PythonLimits::default().max_duration(timeout))
            .sqlite_with_limits(SqliteLimits::default().max_duration(timeout))
            .network(NetworkAllowlist::allow_all())
            .http_transport(Arc::new(NetworkAdapter(request.network)))
            .build();

        let execution = execution::execute(request.command, &mut bash, fs.as_ref()).await?;
        Ok(RunResult { execution })
    }
}
