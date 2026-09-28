//! Provider-neutral code execution and workspace synchronization.
//!
//! The public contract deliberately contains no Bionic database identifiers or
//! Bashkit lifecycle concepts. Callers supply a complete execution snapshot and
//! authorization-scoped callback tools for each run.

use async_trait::async_trait;
use bashkit::{
    Bash, Builtin, BuiltinContext, ExecResult, ExecutionLimits, FileSystem, InMemoryFs,
    PythonExternalFnHandler, PythonLimits, SqliteLimits,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Component, Path};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub const HOME_DIR: &str = "/home/user";
pub const WORK_DIR: &str = "/home/user/work";
pub const OUTPUT_DIR: &str = "/home/user/output";
const SCRIPT_PATH: &str = "/home/user/.runtime/run_python.py";
const MAX_COMMANDS: usize = 1_000;
const MAX_STDOUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_STDERR_BYTES: usize = 512 * 1024;
const MAX_FILE_BYTES: usize = 2 * 1024 * 1024;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxFile {
    pub path: String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceSnapshot {
    pub key: String,
    pub revision: String,
    pub files: Vec<SandboxFile>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspaceDelta {
    pub upserted: Vec<SandboxFile>,
    pub deleted: Vec<String>,
}

#[derive(Clone, Default)]
pub struct CredentialSet {
    values: BTreeMap<String, Credential>,
}

impl fmt::Debug for CredentialSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialSet")
            .field("keys", &self.values.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl CredentialSet {
    pub fn insert(&mut self, key: impl Into<String>, credential: Credential) {
        self.values.insert(key.into(), credential);
    }

    pub fn get(&self, key: &str) -> Option<&Credential> {
        self.values.get(key)
    }
}

#[derive(Clone)]
pub enum Credential {
    Header { name: String, value: String },
    Bearer { token: String },
}

impl fmt::Debug for Credential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header { name, .. } => formatter
                .debug_struct("Header")
                .field("name", name)
                .field("value", &"[REDACTED]")
                .finish(),
            Self::Bearer { .. } => formatter.write_str("Bearer([REDACTED])"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenApiSpec {
    pub name: String,
    pub document: Value,
}

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
        let names = tools
            .iter()
            .filter(|tool| tool.definition().exposure == ToolExposure::Python)
            .map(|tool| tool.definition().name)
            .collect::<Vec<_>>();
        let handler = external_handler(Arc::clone(&tools), Arc::clone(&credentials), fs.clone());

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
                names,
                handler,
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
        let mut bash = builder.build();

        let started = Instant::now();
        let execution = match request.command {
            Command::Shell { script, .. } => {
                let result = bash.exec(&script).await.map_err(error)?;
                from_exec(result, started.elapsed())
            }
            Command::Python { code, .. } => {
                write_checked(fs.as_ref(), SCRIPT_PATH, code.as_bytes()).await?;
                let result = bash
                    .exec("python3 /home/user/.runtime/run_python.py")
                    .await
                    .map_err(error)?;
                from_exec(result, started.elapsed())
            }
            Command::ReadFile { path } => {
                let path = checked_path(&path)?;
                let bytes = fs.read_file(path).await.map_err(error)?;
                if bytes.len() > MAX_FILE_BYTES {
                    return Err(SandboxError(format!("file exceeds {MAX_FILE_BYTES} bytes")));
                }
                let mut result = success(String::new(), started.elapsed());
                result.data = Some(bytes);
                result
            }
            Command::WriteFile { path, contents } => {
                ensure_size(&contents)?;
                write_checked(fs.as_ref(), &path, &contents).await?;
                success(String::new(), started.elapsed())
            }
            Command::EditFile {
                path,
                find,
                replace,
            } => {
                ensure_size(&find)?;
                ensure_size(&replace)?;
                let checked = checked_path(&path)?;
                let original = fs.read_file(checked).await.map_err(error)?;
                let content = String::from_utf8(original).map_err(|err| error(err.to_string()))?;
                let find = String::from_utf8(find).map_err(|err| error(err.to_string()))?;
                let replace = String::from_utf8(replace).map_err(|err| error(err.to_string()))?;
                let count = content.match_indices(&find).count();
                if count != 1 {
                    return Err(SandboxError(format!(
                        "find text must occur exactly once, found {count}"
                    )));
                }
                let updated = content.replacen(&find, &replace, 1);
                ensure_size(updated.as_bytes())?;
                write_checked(fs.as_ref(), &path, updated.as_bytes()).await?;
                success(String::new(), started.elapsed())
            }
        };

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

fn external_handler(
    tools: Arc<Vec<Arc<dyn SandboxTool>>>,
    credentials: Arc<CredentialSet>,
    fs: Arc<dyn FileSystem>,
) -> PythonExternalFnHandler {
    Arc::new(move |name, args, kwargs| {
        let tools = Arc::clone(&tools);
        let credentials = Arc::clone(&credentials);
        let files = FsToolFiles(fs.clone());
        Box::pin(async move {
            let Some(tool) = tools.iter().find(|tool| tool.definition().name == name) else {
                return bashkit::ExtFunctionResult::Error(bashkit::MontyException::new(
                    bashkit::ExcType::ValueError,
                    Some(format!("unknown sandbox tool: {name}")),
                ));
            };
            let definition = tool.definition();
            let arguments = monty_arguments(args, kwargs);
            let context = ToolCallContext {
                files: &files,
                credential: definition
                    .credential
                    .as_deref()
                    .and_then(|key| credentials.get(key)),
            };
            match tool.call(arguments, context).await {
                Ok(value) => bashkit::ExtFunctionResult::Return(json_to_monty(&value)),
                Err(err) => bashkit::ExtFunctionResult::Error(bashkit::MontyException::new(
                    bashkit::ExcType::ValueError,
                    Some(err.to_string()),
                )),
            }
        })
    })
}

struct FsToolFiles(Arc<dyn FileSystem>);

struct CallbackBuiltin {
    tool: Arc<dyn SandboxTool>,
    credentials: Arc<CredentialSet>,
}

#[async_trait]
impl Builtin for CallbackBuiltin {
    async fn execute(&self, ctx: BuiltinContext<'_>) -> bashkit::Result<ExecResult> {
        let definition = self.tool.definition();
        let files = FsToolFiles(ctx.fs);
        let arguments = serde_json::json!({
            "args": ctx.args,
            "stdin": ctx.stdin,
        });
        let context = ToolCallContext {
            files: &files,
            credential: definition
                .credential
                .as_deref()
                .and_then(|key| self.credentials.get(key)),
        };
        match self.tool.call(arguments, context).await {
            Ok(Value::String(value)) => Ok(ExecResult::ok(format!("{value}\n"))),
            Ok(value) => Ok(ExecResult::ok(format!("{value}\n"))),
            Err(error) => Ok(ExecResult::err(format!("{error}\n"), 1)),
        }
    }

    fn llm_hint(&self) -> Option<&'static str> {
        None
    }
}

#[async_trait]
impl ToolFiles for FsToolFiles {
    async fn read(&self, path: &str) -> Result<Vec<u8>, SandboxToolError> {
        checked_path(path).map_err(|err| SandboxToolError(err.to_string()))?;
        self.0
            .read_file(Path::new(path))
            .await
            .map_err(|err| SandboxToolError(err.to_string()))
    }

    async fn write(&self, path: &str, contents: &[u8]) -> Result<(), SandboxToolError> {
        write_checked(self.0.as_ref(), path, contents)
            .await
            .map_err(|err| SandboxToolError(err.to_string()))
    }
}

fn monty_arguments(
    args: Vec<bashkit::MontyObject>,
    kwargs: Vec<(bashkit::MontyObject, bashkit::MontyObject)>,
) -> Value {
    if kwargs.is_empty() {
        return Value::Array(args.iter().map(monty_to_json).collect());
    }
    let mut values = serde_json::Map::new();
    for (key, value) in kwargs {
        if let bashkit::MontyObject::String(key) = key {
            values.insert(key, monty_to_json(&value));
        }
    }
    Value::Object(values)
}

fn monty_to_json(value: &bashkit::MontyObject) -> Value {
    match value {
        bashkit::MontyObject::None => Value::Null,
        bashkit::MontyObject::Bool(value) => Value::Bool(*value),
        bashkit::MontyObject::Int(value) => Value::Number((*value).into()),
        bashkit::MontyObject::BigInt(value) => Value::String(value.to_string()),
        bashkit::MontyObject::Float(value) => serde_json::Number::from_f64(*value)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        bashkit::MontyObject::String(value) | bashkit::MontyObject::Path(value) => {
            Value::String(value.clone())
        }
        bashkit::MontyObject::Bytes(value) => {
            Value::String(String::from_utf8_lossy(value).to_string())
        }
        bashkit::MontyObject::List(values) | bashkit::MontyObject::Tuple(values) => {
            Value::Array(values.iter().map(monty_to_json).collect())
        }
        bashkit::MontyObject::Dict(values) => Value::Object(
            values
                .into_iter()
                .filter_map(|(key, value)| match key {
                    bashkit::MontyObject::String(key) => Some((key.clone(), monty_to_json(value))),
                    _ => None,
                })
                .collect(),
        ),
        _ => Value::String(format!("{value:?}")),
    }
}

fn json_to_monty(value: &Value) -> bashkit::MontyObject {
    match value {
        Value::Null => bashkit::MontyObject::None,
        Value::Bool(value) => bashkit::MontyObject::Bool(*value),
        Value::Number(value) => value
            .as_i64()
            .map(bashkit::MontyObject::Int)
            .or_else(|| value.as_f64().map(bashkit::MontyObject::Float))
            .unwrap_or(bashkit::MontyObject::None),
        Value::String(value) => bashkit::MontyObject::String(value.clone()),
        Value::Array(values) => {
            bashkit::MontyObject::List(values.iter().map(json_to_monty).collect())
        }
        Value::Object(values) => bashkit::MontyObject::Dict(
            values
                .iter()
                .map(|(key, value)| {
                    (
                        bashkit::MontyObject::String(key.clone()),
                        json_to_monty(value),
                    )
                })
                .collect(),
        ),
    }
}

fn command_timeout(command: &Command) -> Duration {
    match command {
        Command::Shell { timeout, .. } | Command::Python { timeout, .. } => *timeout,
        _ => Duration::from_secs(30),
    }
}

async fn seed_files<'a>(
    fs: &dyn FileSystem,
    files: impl Iterator<Item = &'a SandboxFile>,
) -> Result<(), SandboxError> {
    for file in files {
        write_checked(fs, &file.path, &file.contents).await?;
    }
    for dir in [WORK_DIR, OUTPUT_DIR] {
        fs.mkdir(Path::new(dir), true).await.map_err(error)?;
    }
    Ok(())
}

async fn seed_openapi_specs(
    fs: &dyn FileSystem,
    specs: &[OpenApiSpec],
) -> Result<(), SandboxError> {
    for spec in specs {
        let name = spec
            .name
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                    character
                } else {
                    '-'
                }
            })
            .collect::<String>();
        let contents = serde_json::to_vec_pretty(&spec.document).map_err(error)?;
        write_checked(
            fs,
            &format!("{HOME_DIR}/functions/{name}.openapi.json"),
            &contents,
        )
        .await?;
    }
    Ok(())
}

async fn write_checked(
    fs: &dyn FileSystem,
    path: &str,
    contents: &[u8],
) -> Result<(), SandboxError> {
    let path = checked_path(path)?;
    if let Some(parent) = path.parent() {
        fs.mkdir(parent, true).await.map_err(error)?;
    }
    fs.write_file(path, contents).await.map_err(error)
}

fn checked_path(path: &str) -> Result<&Path, SandboxError> {
    let path = Path::new(path);
    if !path.is_absolute() || !path.starts_with(HOME_DIR) {
        return Err(SandboxError("path must be inside /home/user".to_string()));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err(SandboxError("path must not contain . or ..".to_string()));
    }
    Ok(path)
}

fn ensure_size(bytes: &[u8]) -> Result<(), SandboxError> {
    if bytes.len() > MAX_FILE_BYTES {
        Err(SandboxError(format!(
            "content exceeds {MAX_FILE_BYTES} bytes"
        )))
    } else {
        Ok(())
    }
}

fn mutable_files(files: &[SandboxFile]) -> BTreeMap<String, SandboxFile> {
    files
        .iter()
        .filter(|file| is_mutable(&file.path))
        .map(|file| (file.path.clone(), file.clone()))
        .collect()
}

async fn collect_mutable_files(
    fs: &dyn FileSystem,
) -> Result<BTreeMap<String, SandboxFile>, SandboxError> {
    let mut files = BTreeMap::new();
    let mut pending = vec![
        Path::new(WORK_DIR).to_path_buf(),
        Path::new(OUTPUT_DIR).to_path_buf(),
    ];
    while let Some(dir) = pending.pop() {
        for entry in fs.read_dir(&dir).await.map_err(error)? {
            let path = dir.join(entry.name);
            if entry.metadata.file_type == bashkit::FileType::Directory {
                pending.push(path);
            } else if entry.metadata.file_type == bashkit::FileType::File {
                let path = path.to_string_lossy().to_string();
                let contents = fs.read_file(Path::new(&path)).await.map_err(error)?;
                files.insert(path.clone(), SandboxFile { path, contents });
            }
        }
    }
    Ok(files)
}

fn diff_workspace(
    before: BTreeMap<String, SandboxFile>,
    after: &BTreeMap<String, SandboxFile>,
) -> WorkspaceDelta {
    let upserted = after
        .iter()
        .filter(|(path, file)| before.get(*path) != Some(*file))
        .map(|(_, file)| file.clone())
        .collect();
    let after_paths = after.keys().cloned().collect::<BTreeSet<_>>();
    let deleted = before
        .keys()
        .filter(|path| !after_paths.contains(*path))
        .cloned()
        .collect();
    WorkspaceDelta { upserted, deleted }
}

fn is_mutable(path: &str) -> bool {
    path.starts_with(&format!("{WORK_DIR}/")) || path.starts_with(&format!("{OUTPUT_DIR}/"))
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

fn success(stdout: String, elapsed: Duration) -> ExecutionResult {
    ExecutionResult {
        stdout,
        stderr: String::new(),
        exit_code: 0,
        duration_ms: elapsed.as_millis(),
        stdout_truncated: false,
        stderr_truncated: false,
        data: None,
    }
}

fn error(error: impl ToString) -> SandboxError {
    SandboxError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoTool;

    #[async_trait]
    impl SandboxTool for EchoTool {
        fn definition(&self) -> SandboxToolDefinition {
            SandboxToolDefinition {
                name: "echo_callback".to_string(),
                description: "Echo a value".to_string(),
                parameters: serde_json::json!({"type": "object"}),
                exposure: ToolExposure::Python,
                credential: None,
            }
        }

        async fn call(
            &self,
            arguments: Value,
            _context: ToolCallContext<'_>,
        ) -> Result<Value, SandboxToolError> {
            Ok(arguments.get("value").cloned().unwrap_or(Value::Null))
        }
    }

    #[tokio::test]
    async fn workspace_changes_are_reported() {
        let result = BashkitSandbox
            .run(RunRequest {
                command: Command::WriteFile {
                    path: "/home/user/work/example.txt".to_string(),
                    contents: b"new".to_vec(),
                },
                skills: vec![],
                openapi_specs: vec![],
                credentials: CredentialSet::default(),
                workspace: WorkspaceSnapshot::default(),
                tools: vec![],
            })
            .await
            .unwrap();
        assert_eq!(result.workspace_changes.upserted.len(), 1);
        assert_eq!(result.workspace_changes.upserted[0].contents, b"new");
    }

    #[tokio::test]
    async fn python_calls_supplied_callback_tools() {
        let result = BashkitSandbox
            .run(RunRequest {
                command: Command::Python {
                    code: "print(echo_callback(value='scheduled'))".to_string(),
                    timeout: Duration::from_secs(5),
                },
                skills: vec![],
                openapi_specs: vec![],
                credentials: CredentialSet::default(),
                workspace: WorkspaceSnapshot::default(),
                tools: vec![Arc::new(EchoTool)],
            })
            .await
            .unwrap();
        assert_eq!(result.execution.stdout, "scheduled\n");
        assert_eq!(result.execution.exit_code, 0);
    }

    #[tokio::test]
    async fn openapi_specs_are_exposed_in_the_filesystem() {
        let result = BashkitSandbox
            .run(RunRequest {
                command: Command::ReadFile {
                    path: "/home/user/functions/calendar.openapi.json".to_string(),
                },
                skills: vec![],
                openapi_specs: vec![OpenApiSpec {
                    name: "calendar".to_string(),
                    document: serde_json::json!({"openapi": "3.0.0"}),
                }],
                credentials: CredentialSet::default(),
                workspace: WorkspaceSnapshot::default(),
                tools: vec![],
            })
            .await
            .unwrap();
        let contents = result.execution.data.unwrap();
        assert!(String::from_utf8(contents).unwrap().contains("3.0.0"));
    }

    #[test]
    fn credentials_do_not_debug_secrets() {
        let mut credentials = CredentialSet::default();
        credentials.insert(
            "integration",
            Credential::Bearer {
                token: "secret".to_string(),
            },
        );
        let debug = format!("{credentials:?}");
        assert!(!debug.contains("secret"));
    }
}
