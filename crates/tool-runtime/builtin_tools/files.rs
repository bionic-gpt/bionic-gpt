use crate::builtin_tools::bashkit::{
    persist_outputs, seeded_filesystem, OutputEntry, MAX_FILE_TOOL_BYTES,
};
use crate::{ToolDyn, ToolError};
use bashkit::FileSystem;
use db::Pool;
use rig::wasm_compat::WasmBoxedFuture;
use sandbox::Sandbox;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

const HOME_DIR: &str = "/home/user";
const MAX_PATH_BYTES: usize = 4096;

#[derive(Clone, Copy)]
enum Operation {
    Read,
    Write,
    Edit,
    Python,
}

#[derive(Clone)]
pub struct FileTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
    model_id: i32,
}

impl FileTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64, model_id: i32) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
            model_id,
        }
    }

    pub fn read(self) -> ReadFileTool {
        ReadFileTool(self)
    }

    pub fn write(self) -> WriteFileTool {
        WriteFileTool(self)
    }

    pub fn edit(self) -> EditFileTool {
        EditFileTool(self)
    }

    pub fn python(self) -> RunPythonTool {
        RunPythonTool(self)
    }
}

impl ToolDyn for FileTool {
    fn name(&self) -> String {
        "file_tools".to_string()
    }

    fn description(&self) -> String {
        "Read and edit files in the virtual filesystem, or run Python in the sandbox.".to_string()
    }

    fn parameters(&self) -> Value {
        json!({})
    }

    fn call<'a>(&'a self, _args: String) -> WasmBoxedFuture<'a, Result<String, ToolError>> {
        Box::pin(async move {
            Err(ToolError::ToolCallError(Box::new(std::io::Error::other(
                "file_tools is an internal grouping and is not directly callable",
            ))))
        })
    }
}

pub struct ReadFileTool(FileTool);
pub struct WriteFileTool(FileTool);
pub struct EditFileTool(FileTool);
pub struct RunPythonTool(FileTool);

macro_rules! impl_file_tool {
    ($type:ident, $operation:expr, $definition:ident) => {
        impl ToolDyn for $type {
            fn name(&self) -> String {
                $definition().name
            }

            fn description(&self) -> String {
                $definition().description
            }

            fn parameters(&self) -> Value {
                $definition().parameters
            }

            fn call<'a>(&'a self, args: String) -> WasmBoxedFuture<'a, Result<String, ToolError>> {
                Box::pin(async move {
                    execute_operation(&self.0, $operation, &args)
                        .await
                        .map_err(ToolError::ToolCallError)
                })
            }
        }
    };
}

impl_file_tool!(ReadFileTool, Operation::Read, get_read_file_definition);
impl_file_tool!(WriteFileTool, Operation::Write, get_write_file_definition);
impl_file_tool!(EditFileTool, Operation::Edit, get_edit_file_definition);
impl_file_tool!(RunPythonTool, Operation::Python, get_run_python_definition);

pub fn get_read_file_definition() -> crate::types::ToolDefinition {
    definition(
        "read_file",
        "Read a UTF-8 or binary file from the virtual filesystem.",
        json!({
            "type": "object",
            "properties": {"path": {"type": "string"}},
            "required": ["path"]
        }),
    )
}

pub fn get_write_file_definition() -> crate::types::ToolDefinition {
    definition(
        "write_file",
        "Write a file to the virtual filesystem. Files under /home/user/work and /home/user/output persist; only output files appear in chat.",
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "content": {"type": "string"}
            },
            "required": ["path", "content"]
        }),
    )
}

pub fn get_edit_file_definition() -> crate::types::ToolDefinition {
    definition(
        "edit_file",
        "Replace exactly one occurrence in a virtual filesystem file.",
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "find": {"type": "string"},
                "replace": {"type": "string"}
            },
            "required": ["path", "find", "replace"]
        }),
    )
}

pub fn get_run_python_definition() -> crate::types::ToolDefinition {
    definition(
        "run_python",
        "Run dependency-free Python in Monty with the virtual filesystem and integrations available.",
        json!({
            "type": "object",
            "properties": {"code": {"type": "string"}},
            "required": ["code"]
        }),
    )
}

fn definition(name: &str, description: &str, parameters: Value) -> crate::types::ToolDefinition {
    crate::types::ToolDefinition {
        name: name.to_string(),
        description: description.to_string(),
        parameters,
    }
}

#[derive(Debug, Deserialize)]
struct ReadArgs {
    path: String,
}

#[derive(Debug, Deserialize)]
struct WriteArgs {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct EditArgs {
    path: String,
    find: String,
    replace: String,
}

#[derive(Debug, Deserialize)]
struct PythonArgs {
    code: String,
}

async fn execute_operation(
    tool: &FileTool,
    operation: Operation,
    args: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let fs = seeded_filesystem(&tool.pool, &tool.sub, tool.conversation_id, tool.model_id)
        .await
        .map_err(|error| std::io::Error::other(error.to_string()))?;

    match operation {
        Operation::Read => {
            let arguments: ReadArgs = serde_json::from_str(args)?;
            let path = checked_path(&arguments.path)?;
            let result = run_sandbox_command(
                tool,
                fs.as_ref(),
                sandbox::Command::ReadFile {
                    path: path.to_string_lossy().to_string(),
                },
                Vec::new(),
            )
            .await?;
            let bytes = result.execution.data.unwrap_or_default();
            if bytes.len() > MAX_FILE_TOOL_BYTES {
                return Err(format!("file exceeds {MAX_FILE_TOOL_BYTES} bytes").into());
            }
            let result = match String::from_utf8(bytes) {
                Ok(content) => json!({"path": path, "content": content, "encoding": "utf-8"}),
                Err(error) => json!({
                    "path": path,
                    "content": base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        error.into_bytes(),
                    ),
                    "encoding": "base64"
                }),
            };
            Ok(result.to_string())
        }
        Operation::Write => {
            let arguments: WriteArgs = serde_json::from_str(args)?;
            let path = checked_path(&arguments.path)?;
            ensure_size(arguments.content.as_bytes())?;
            run_sandbox_command(
                tool,
                fs.as_ref(),
                sandbox::Command::WriteFile {
                    path: path.to_string_lossy().to_string(),
                    contents: arguments.content.into_bytes(),
                },
                Vec::new(),
            )
            .await?;
            let outputs = persist_output_if_needed(tool, &fs).await?;
            Ok(json!({"path": path, "written": true, "outputs": outputs}).to_string())
        }
        Operation::Edit => {
            let arguments: EditArgs = serde_json::from_str(args)?;
            let path = checked_path(&arguments.path)?;
            ensure_size(arguments.find.as_bytes())?;
            ensure_size(arguments.replace.as_bytes())?;
            run_sandbox_command(
                tool,
                fs.as_ref(),
                sandbox::Command::EditFile {
                    path: path.to_string_lossy().to_string(),
                    find: arguments.find.into_bytes(),
                    replace: arguments.replace.into_bytes(),
                },
                Vec::new(),
            )
            .await?;
            let outputs = persist_output_if_needed(tool, &fs).await?;
            Ok(json!({"path": path, "edited": true, "outputs": outputs}).to_string())
        }
        Operation::Python => {
            let arguments: PythonArgs = serde_json::from_str(args)?;
            ensure_size(arguments.code.as_bytes())?;
            let registry = std::sync::Arc::new(
                crate::builtin_tools::monty::RuntimeFunctionRegistry::load_for_conversation(
                    &tool.pool,
                    &tool.sub,
                    tool.conversation_id,
                )
                .await?,
            );
            let workspace = sandbox::WorkspaceSnapshot {
                key: tool.conversation_id.to_string(),
                revision: String::new(),
                files: snapshot_files(fs.as_ref(), Path::new(HOME_DIR)).await?,
            };
            let result = sandbox::BashkitSandbox
                .run(sandbox::RunRequest {
                    command: sandbox::Command::Python {
                        code: arguments.code,
                        timeout: Duration::from_secs(30),
                    },
                    skills: Vec::new(),
                    openapi_specs: Vec::new(),
                    credentials: sandbox::CredentialSet::default(),
                    workspace,
                    tools: registry.sandbox_tools(),
                })
                .await?;
            apply_workspace_delta(fs.as_ref(), &result.workspace_changes).await?;
            let outputs = persist_output_if_needed(tool, &fs).await?;
            Ok(json!({
                "stdout": result.execution.stdout,
                "stderr": result.execution.stderr,
                "exit_code": result.execution.exit_code,
                "outputs": outputs
            })
            .to_string())
        }
    }
}

async fn run_sandbox_command(
    tool: &FileTool,
    fs: &dyn FileSystem,
    command: sandbox::Command,
    tools: Vec<Arc<dyn sandbox::SandboxTool>>,
) -> Result<sandbox::RunResult, Box<dyn std::error::Error + Send + Sync>> {
    let result = sandbox::BashkitSandbox
        .run(sandbox::RunRequest {
            command,
            skills: Vec::new(),
            openapi_specs: Vec::new(),
            credentials: sandbox::CredentialSet::default(),
            workspace: sandbox::WorkspaceSnapshot {
                key: tool.conversation_id.to_string(),
                revision: String::new(),
                files: snapshot_files(fs, Path::new(HOME_DIR)).await?,
            },
            tools,
        })
        .await?;
    apply_workspace_delta(fs, &result.workspace_changes).await?;
    Ok(result)
}

async fn snapshot_files(
    fs: &dyn FileSystem,
    root: &Path,
) -> Result<Vec<sandbox::SandboxFile>, Box<dyn std::error::Error + Send + Sync>> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs.read_dir(&directory).await? {
            let path = directory.join(entry.name);
            if entry.metadata.file_type == bashkit::FileType::Directory {
                pending.push(path);
            } else if entry.metadata.file_type == bashkit::FileType::File {
                files.push(sandbox::SandboxFile {
                    path: path.to_string_lossy().to_string(),
                    contents: fs.read_file(&path).await?,
                });
            }
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

async fn apply_workspace_delta(
    fs: &dyn FileSystem,
    delta: &sandbox::WorkspaceDelta,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for path in &delta.deleted {
        if fs.exists(Path::new(path)).await? {
            fs.remove(Path::new(path), false).await?;
        }
    }
    for file in &delta.upserted {
        let path = Path::new(&file.path);
        if let Some(parent) = path.parent() {
            fs.mkdir(parent, true).await?;
        }
        fs.write_file(path, &file.contents).await?;
    }
    Ok(())
}

#[cfg(test)]
fn replace_once(
    content: &str,
    find: &str,
    replace: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let matches = content.match_indices(find).count();
    if matches != 1 {
        return Err(format!("find text must occur exactly once, found {matches}").into());
    }
    Ok(content.replacen(find, replace, 1))
}

fn checked_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    if path.len() > MAX_PATH_BYTES {
        return Err("path is too long".into());
    }
    let path = Path::new(path);
    if !path.is_absolute() || !path.starts_with(HOME_DIR) {
        return Err("path must be inside /home/user".into());
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err("path must not contain . or ..".into());
    }
    Ok(path.to_path_buf())
}

fn ensure_size(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if bytes.len() > MAX_FILE_TOOL_BYTES {
        return Err(format!("content exceeds {MAX_FILE_TOOL_BYTES} bytes").into());
    }
    Ok(())
}

async fn persist_output_if_needed(
    tool: &FileTool,
    fs: &Arc<dyn FileSystem>,
) -> Result<Vec<OutputEntry>, Box<dyn std::error::Error + Send + Sync>> {
    persist_outputs(&tool.pool, &tool.sub, tool.conversation_id, fs.as_ref())
        .await
        .map_err(|error| std::io::Error::other(error.to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_are_limited_to_the_virtual_home() {
        assert!(checked_path("/home/user/output/file.txt").is_ok());
        assert!(checked_path("/home/user/work/file.txt").is_ok());
        assert!(checked_path("/tmp/file.txt").is_err());
        assert!(checked_path("/home/user/../etc/passwd").is_err());
    }

    #[test]
    fn edit_requires_exactly_one_match() {
        assert_eq!(
            replace_once("hello world", "world", "Bionic").unwrap(),
            "hello Bionic"
        );
        assert!(replace_once("same same", "same", "new").is_err());
        assert!(replace_once("hello", "missing", "new").is_err());
    }

    #[test]
    fn definitions_use_separate_file_tool_names() {
        assert_eq!(get_read_file_definition().name, "read_file");
        assert_eq!(get_write_file_definition().name, "write_file");
        assert_eq!(get_edit_file_definition().name, "edit_file");
        assert_eq!(get_run_python_definition().name, "run_python");
    }
}
