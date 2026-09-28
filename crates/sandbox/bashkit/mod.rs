mod callbacks;
mod execution;
mod filesystem;

use crate::{
    diff_workspace, mutable_files, Artifact, RunRequest, RunResult, Sandbox, SandboxError,
    ToolExposure, HOME_DIR, OUTPUT_DIR,
};
use async_trait::async_trait;
use bashkit::{Bash, ExecutionLimits, FileSystem, InMemoryFs, PythonLimits, SqliteLimits};
use callbacks::{python_handler, python_names, CallbackBuiltin};
use execution::{command_timeout, MAX_COMMANDS, MAX_STDERR_BYTES, MAX_STDOUT_BYTES};
use filesystem::{collect_mutable_files, seed_files, seed_openapi_specs};
use std::sync::Arc;

#[derive(Default)]
pub struct BashkitSandbox;

#[async_trait]
impl Sandbox for BashkitSandbox {
    async fn run(&self, request: RunRequest) -> Result<RunResult, SandboxError> {
        let before = mutable_files(&request.workspace.files);
        let fs: Arc<dyn FileSystem> = Arc::new(InMemoryFs::new());
        seed_files(fs.as_ref(), request.workspace.files.iter()).await?;
        seed_files(fs.as_ref(), request.skills.iter()).await?;
        seed_openapi_specs(fs.as_ref(), &request.openapi_specs).await?;

        let tools = Arc::new(request.tools);
        let credentials = Arc::new(request.credentials);
        let timeout = command_timeout(&request.command);
        let mut builder = Bash::builder()
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
            .python_with_external_handler(
                PythonLimits::default().max_duration(timeout),
                python_names(&tools),
                python_handler(Arc::clone(&tools), Arc::clone(&credentials), fs.clone()),
            )
            .sqlite_with_limits(SqliteLimits::default().max_duration(timeout));

        for tool in tools
            .iter()
            .filter(|tool| tool.definition().exposure == ToolExposure::Shell)
        {
            let definition = tool.definition();
            builder = builder.builtin(
                definition.name,
                Box::new(CallbackBuiltin {
                    tool: Arc::clone(tool),
                    credentials: Arc::clone(&credentials),
                }),
            );
        }

        let execution =
            execution::execute(request.command, &mut builder.build(), fs.as_ref()).await?;
        let after = collect_mutable_files(fs.as_ref()).await?;
        let workspace_changes = diff_workspace(before, &after);
        let artifacts = after
            .values()
            .filter(|file| file.path.starts_with(&format!("{OUTPUT_DIR}/")))
            .map(|file| Artifact {
                path: file.path.clone(),
                size: file.contents.len() as u64,
            })
            .collect();

        Ok(RunResult {
            execution,
            workspace_changes,
            artifacts,
        })
    }
}
