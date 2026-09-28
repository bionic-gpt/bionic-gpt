use super::filesystem::{checked_path, write_checked};
use crate::{error, Command, ExecutionResult, SandboxError};
use bashkit::{Bash, FileSystem};
use std::time::{Duration, Instant};

pub(super) const SCRIPT_PATH: &str = "/home/user/.runtime/run_python.py";
pub(super) const MAX_COMMANDS: usize = 1_000;
pub(super) const MAX_STDOUT_BYTES: usize = 2 * 1024 * 1024;
pub(super) const MAX_STDERR_BYTES: usize = 512 * 1024;
const MAX_FILE_BYTES: usize = 2 * 1024 * 1024;

pub(super) fn command_timeout(command: &Command) -> Duration {
    match command {
        Command::Shell { timeout, .. } | Command::Python { timeout, .. } => *timeout,
        _ => Duration::from_secs(30),
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
            write_checked(fs, SCRIPT_PATH, code.as_bytes()).await?;
            let result = bash
                .exec("python3 /home/user/.runtime/run_python.py")
                .await
                .map_err(error)?;
            Ok(from_exec(result, started.elapsed()))
        }
        Command::ReadFile { path } => {
            let path = checked_path(&path)?;
            let bytes = fs.read_file(path).await.map_err(error)?;
            ensure_size(&bytes, "file exceeds")?;
            let mut result = success(started.elapsed());
            result.data = Some(bytes);
            Ok(result)
        }
        Command::WriteFile { path, contents } => {
            ensure_size(&contents, "content exceeds")?;
            write_checked(fs, &path, &contents).await?;
            Ok(success(started.elapsed()))
        }
        Command::EditFile {
            path,
            find,
            replace,
        } => {
            ensure_size(&find, "content exceeds")?;
            ensure_size(&replace, "content exceeds")?;
            let checked = checked_path(&path)?;
            let original = fs.read_file(checked).await.map_err(error)?;
            let content = String::from_utf8(original).map_err(error)?;
            let find = String::from_utf8(find).map_err(error)?;
            let replace = String::from_utf8(replace).map_err(error)?;
            let count = content.match_indices(&find).count();
            if count != 1 {
                return Err(SandboxError(format!(
                    "find text must occur exactly once, found {count}"
                )));
            }
            let updated = content.replacen(&find, &replace, 1);
            ensure_size(updated.as_bytes(), "content exceeds")?;
            write_checked(fs, &path, updated.as_bytes()).await?;
            Ok(success(started.elapsed()))
        }
    }
}

fn ensure_size(bytes: &[u8], message: &str) -> Result<(), SandboxError> {
    if bytes.len() > MAX_FILE_BYTES {
        Err(SandboxError(format!("{message} {MAX_FILE_BYTES} bytes")))
    } else {
        Ok(())
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
        data: None,
    }
}

fn success(elapsed: Duration) -> ExecutionResult {
    ExecutionResult {
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
        duration_ms: elapsed.as_millis(),
        stdout_truncated: false,
        stderr_truncated: false,
        data: None,
    }
}
