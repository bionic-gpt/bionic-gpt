use crate::{error, Command, ExecutionResult, SandboxError};
use bashkit::{Bash, FileSystem};
use std::time::{Duration, Instant};

pub(super) const SCRIPT_PATH: &str = "/home/user/.runtime/run_python.py";
pub(super) const MAX_COMMANDS: usize = 1_000;
pub(super) const MAX_STDOUT_BYTES: usize = 2 * 1024 * 1024;
pub(super) const MAX_STDERR_BYTES: usize = 512 * 1024;

pub(super) fn command_timeout(command: &Command) -> Duration {
    match command {
        Command::Shell { timeout, .. } | Command::Python { timeout, .. } => *timeout,
    }
}

pub(super) async fn execute(
    command: Command,
    bash: &mut Bash,
    fs: &dyn FileSystem,
) -> Result<ExecutionResult, SandboxError> {
    let started = Instant::now();
    match command {
        Command::Shell { script, .. } => {
            let result = bash.exec(&script).await.map_err(error)?;
            Ok(from_exec(result, started.elapsed()))
        }
        Command::Python { code, .. } => {
            fs.mkdir(std::path::Path::new("/home/user/.runtime"), true)
                .await
                .map_err(error)?;
            fs.write_file(std::path::Path::new(SCRIPT_PATH), code.as_bytes())
                .await
                .map_err(error)?;
            let result = bash
                .exec("python3 /home/user/.runtime/run_python.py")
                .await
                .map_err(error)?;
            Ok(from_exec(result, started.elapsed()))
        }
    }
}

fn from_exec(result: bashkit::ExecResult, elapsed: Duration) -> ExecutionResult {
    ExecutionResult {
        stdout: result.stdout,
        stderr: result.stderr,
        exit_code: result.exit_code,
        duration_ms: elapsed.as_millis(),
        stdout_truncated: result.stdout_truncated,
        stderr_truncated: result.stderr_truncated,
    }
}
