use crate::builtin_tools::bashkit::{persist_outputs, seeded_filesystem, MAX_FILE_TOOL_BYTES};
use crate::{ToolDyn, ToolError};
use bashkit::{Bash, ExecutionLimits, FileSystem, PythonLimits};
use db::Pool;
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

const HOME_DIR: &str = "/home/user";
const SCRIPT_PATH: &str = "/home/user/.runtime/run_python.py";
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
        "Write a file to the virtual filesystem. Files under /home/user/output persist.",
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
            let bytes = fs.read_file(&path).await?;
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
            write_file(&fs, &path, arguments.content.as_bytes()).await?;
            persist_output_if_needed(tool, &fs).await?;
            Ok(json!({"path": path, "written": true}).to_string())
        }
        Operation::Edit => {
            let arguments: EditArgs = serde_json::from_str(args)?;
            let path = checked_path(&arguments.path)?;
            ensure_size(arguments.find.as_bytes())?;
            ensure_size(arguments.replace.as_bytes())?;
            let original = fs.read_file(&path).await?;
            let content = String::from_utf8(original)?;
            let updated = replace_once(&content, &arguments.find, &arguments.replace)?;
            ensure_size(updated.as_bytes())?;
            write_file(&fs, &path, updated.as_bytes()).await?;
            persist_output_if_needed(tool, &fs).await?;
            Ok(json!({"path": path, "edited": true}).to_string())
        }
        Operation::Python => {
            let arguments: PythonArgs = serde_json::from_str(args)?;
            ensure_size(arguments.code.as_bytes())?;
            let script_path = Path::new(SCRIPT_PATH);
            write_file(&fs, script_path, arguments.code.as_bytes()).await?;
            let registry = std::sync::Arc::new(
                crate::builtin_tools::monty::RuntimeFunctionRegistry::load_for_conversation(
                    &tool.pool,
                    &tool.sub,
                    tool.conversation_id,
                )
                .await?,
            );
            let mut bash = Bash::builder()
                .fs(fs.clone())
                .username("user")
                .hostname("bashkit")
                .cwd(HOME_DIR)
                .env("BASHKIT_ALLOW_INPROCESS_PYTHON", "1")
                .limits(ExecutionLimits::default())
                .python_with_external_handler(
                    PythonLimits::default().max_duration(Duration::from_secs(30)),
                    registry.external_function_names(),
                    registry.python_external_handler_with_fs(fs.clone()),
                )
                .build();
            let result = bash
                .exec("python3 /home/user/.runtime/run_python.py")
                .await?;
            persist_output_if_needed(tool, &fs).await?;
            Ok(json!({
                "stdout": result.stdout,
                "stderr": result.stderr,
                "exit_code": result.exit_code
            })
            .to_string())
        }
    }
}

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

async fn write_file(
    fs: &Arc<dyn FileSystem>,
    path: &Path,
    contents: &[u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(parent) = path.parent() {
        fs.mkdir(parent, true).await?;
    }
    fs.write_file(path, contents).await?;
    Ok(())
}

async fn persist_output_if_needed(
    tool: &FileTool,
    fs: &Arc<dyn FileSystem>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    persist_outputs(&tool.pool, &tool.sub, tool.conversation_id, fs.as_ref())
        .await
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_are_limited_to_the_virtual_home() {
        assert!(checked_path("/home/user/output/file.txt").is_ok());
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
