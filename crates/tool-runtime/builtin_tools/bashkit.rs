use crate::skills;
use crate::types::ToolDefinition;
use bashkit::{
    async_trait, Bash, Builtin, BuiltinContext, ExecResult, ExecutionLimits, FileSystem, FileType,
};
use db::{queries, Pool, Transaction};
use object_storage::StorageConfig;
use rig::client::EmbeddingsClient;
use rig::embeddings::EmbeddingModel;
use rig::providers::{ollama, openai};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT_MS: u64 = 5_000;
const MAX_TIMEOUT_MS: u64 = 30_000;
const MAX_COMMANDS: usize = 1_000;
const MAX_STDOUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_STDERR_BYTES: usize = 512 * 1024;
const CHUNKS_PER_DOCUMENT_LIMIT: i64 = 1_000;
const HOME_DIR: &str = "/home/user";
const SKILLS_DIR: &str = "/home/user/skills";
const FUNCTIONS_DIR: &str = "/home/user/functions";
const DATASETS_DIR: &str = "/home/user/datasets";
const ATTACHMENTS_DIR: &str = "/home/user/attachments";
const OUTPUT_DIR: &str = "/home/user/output";
const MAX_OUTPUT_FILES: usize = 50;
const MAX_OUTPUT_FILE_BYTES: u64 = 5 * 1024 * 1024;

#[derive(Debug, Deserialize)]
struct RunBashArgs {
    commands: String,
    timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
struct DatasetManifest {
    datasets: Vec<DatasetEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct DatasetEntry {
    dataset_id: i32,
    name: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct DatasetMetadata {
    dataset_id: i32,
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct DatasetFiles {
    files: Vec<FileEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct FileEntry {
    document_id: i32,
    name: String,
    size: i32,
    chunks: i64,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct FileMetadata {
    document_id: i32,
    dataset_id: i32,
    name: String,
    size: i32,
    chunks: i64,
}

#[derive(Debug, Clone, Serialize)]
struct AttachmentManifest {
    files: Vec<AttachmentEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct AttachmentEntry {
    file_id: i32,
    name: String,
    path: String,
    mime_type: String,
    size: i64,
}

#[derive(Debug, Clone, Serialize)]
struct OutputEntry {
    id: i32,
    path: String,
    file_name: String,
    mime_type: String,
    size: i64,
}

#[derive(Debug, Clone)]
struct ChunkPath {
    dataset_id: i32,
    document_id: i32,
    chunk_id: i32,
}

impl ChunkPath {
    fn vfs_path(&self) -> String {
        format!(
            "{}/{}/files/{}/chunks/{}.txt",
            DATASETS_DIR, self.dataset_id, self.document_id, self.chunk_id
        )
    }
}

pub struct BashkitTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
    prompt_id: i32,
}

impl BashkitTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64, prompt_id: i32) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
            prompt_id,
        }
    }
}

impl ToolDyn for BashkitTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn description(&self) -> String {
        get_tool_definition().description
    }

    fn parameters(&self) -> Value {
        get_tool_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: RunBashArgs =
                serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let result = execute_run_bash(self, arguments).await.map_err(|err| {
                ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
            })?;
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "run_bash".to_string(),
        description: "Run shell commands in Bashkit, an in-process sandboxed bash runtime with a virtual filesystem. Use /home/user/attachments to inspect uploaded chat files, /home/user/skills to read available skill instructions, /home/user/functions to discover callable integration functions, and /home/user/datasets to inspect assistant datasets. Use /home/user/output for generated files that should persist across tool calls and appear in the chat. Use rag-search 'query' to find relevant chunks and rag-read /home/user/datasets/.../chunks/<id>.txt to read a chunk. The filesystem is fresh for each call except /home/user/output, network access is disabled, and host files are not mounted.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "commands": {
                    "type": "string",
                    "description": "Bash commands to run in the sandbox."
                },
                "timeout_ms": {
                    "type": "integer",
                    "minimum": 100,
                    "maximum": MAX_TIMEOUT_MS,
                    "description": "Optional execution timeout in milliseconds. Defaults to 5000."
                }
            },
            "required": ["commands"]
        }),
    }
}

pub fn preview_vfs_tree(
    skill_summaries: &[db::queries::skills::SkillSummary],
    function_files: &[crate::builtin_tools::monty::RuntimeFunctionFile],
) -> String {
    let skill_dirs = skill_summaries
        .iter()
        .map(|skill| {
            skills::skill_vfs_directory(skill.skill_id, &skill.skill_name, skill.is_system)
                .trim_start_matches("/home/user/skills/")
                .to_string()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let function_files = function_files
        .iter()
        .map(|file| {
            file.path
                .trim_start_matches("/home/user/functions/")
                .to_string()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut tree = String::from(
        "/home/user\n\
|-- attachments                 # conversation scoped\n\
|   |-- index.json\n\
|   `-- <uploaded_file_name>\n\
|-- datasets                    # prompt/assistant scoped\n\
|   |-- index.json\n\
|   `-- <dataset_id>\n\
|       |-- metadata.json\n\
|       `-- files\n\
|           `-- <document_id>\n\
|               |-- metadata.json\n\
|               `-- chunks\n\
|                   `-- <chunk_id>.txt\n\
|-- output                      # persists for this conversation\n\
|   `-- <generated_file_or_directory>\n\
|-- functions                   # connected integration catalogues\n",
    );

    if function_files.is_empty() {
        tree.push_str("|   `-- <no connected integrations>\n");
    } else {
        for (index, file) in function_files.iter().enumerate() {
            let branch = if index + 1 == function_files.len() {
                "`--"
            } else {
                "|--"
            };
            tree.push_str(&format!("|   {branch} {file}\n"));
        }
    }

    tree.push_str("`-- skills                      # current visible skills\n");

    if skill_dirs.is_empty() {
        tree.push_str("    `-- <no visible skills>\n");
    } else {
        for (index, dir) in skill_dirs.iter().enumerate() {
            let is_last = index + 1 == skill_dirs.len();
            let branch = if is_last { "`--" } else { "|--" };
            let child_prefix = if is_last { "   " } else { "|  " };
            tree.push_str(&format!("    {branch} {dir}\n"));
            tree.push_str(&format!("    {child_prefix} `-- SKILL.md\n"));
        }
    }

    tree.push_str(
        "\nOmitted from this preview: uploaded file contents, dataset chunk text, generated output contents, secrets, and tokens.",
    );

    tree
}

async fn execute_run_bash(
    tool: &BashkitTool,
    arguments: RunBashArgs,
) -> Result<Value, serde_json::Value> {
    if arguments.commands.trim().is_empty() {
        return Ok(json!({"error": "commands is required"}));
    }

    let timeout = arguments
        .timeout_ms
        .unwrap_or(DEFAULT_TIMEOUT_MS)
        .clamp(100, MAX_TIMEOUT_MS);

    let started = Instant::now();
    let mut bash = Bash::builder()
        .username("user")
        .hostname("bashkit")
        .cwd(HOME_DIR)
        .limits(
            ExecutionLimits::new()
                .timeout(Duration::from_millis(timeout))
                .max_commands(MAX_COMMANDS)
                .max_stdout_bytes(MAX_STDOUT_BYTES)
                .max_stderr_bytes(MAX_STDERR_BYTES),
        )
        .builtin(
            "rag-search",
            Box::new(RagSearchBuiltin {
                pool: tool.pool.clone(),
                sub: tool.sub.clone(),
                conversation_id: tool.conversation_id,
                prompt_id: tool.prompt_id,
            }),
        )
        .builtin("rag-read", Box::new(RagReadBuiltin))
        .build();

    seed_custom_skills(&tool.pool, &tool.sub, &bash).await?;
    seed_function_catalogue(&tool.pool, &tool.sub, tool.conversation_id, &bash).await?;
    seed_datasets(
        &tool.pool,
        &tool.sub,
        tool.prompt_id,
        tool.conversation_id,
        &bash,
    )
    .await?;
    seed_attachments(&tool.pool, &tool.sub, tool.conversation_id, &bash).await?;
    seed_outputs(&tool.pool, &tool.sub, tool.conversation_id, &bash).await?;

    let result = tokio::time::timeout(
        Duration::from_millis(timeout),
        bash.exec(&arguments.commands),
    )
    .await
    .map_err(|_| json!({"error": "bash execution timed out"}))?
    .map_err(|err| json!({"error": "bash execution failed", "details": err.to_string()}))?;

    let output_sync_result =
        persist_outputs(&tool.pool, &tool.sub, tool.conversation_id, &bash).await;

    let mut response = json!({
        "stdout": result.stdout,
        "stderr": result.stderr,
        "exit_code": result.exit_code,
        "duration_ms": started.elapsed().as_millis(),
        "stdout_truncated": result.stdout_truncated,
        "stderr_truncated": result.stderr_truncated
    });

    match output_sync_result {
        Ok(outputs) => response["outputs"] = json!(outputs),
        Err(err) => response["output_error"] = err,
    }

    Ok(response)
}

async fn seed_custom_skills(pool: &Pool, sub: &str, bash: &Bash) -> Result<(), serde_json::Value> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let files = queries::skills::visible_skill_files()
        .bind(&transaction)
        .all()
        .await
        .map_err(|e| json!({"error": "Failed to get skills", "details": e.to_string()}))?;

    let fs = bash.fs();
    fs.mkdir(Path::new(SKILLS_DIR), true).await.map_err(
        |e| json!({"error": "Failed to seed Bashkit skills directory", "details": e.to_string()}),
    )?;

    for file in skills::runtime_skill_files(files) {
        let path = Path::new(&file.path);
        if let Some(parent) = path.parent() {
            fs.mkdir(parent, true)
                .await
                .map_err(|e| json!({"error": "Failed to seed Bashkit skill directory", "details": e.to_string()}))?;
        }
        fs.write_file(path, &file.contents).await.map_err(
            |e| json!({"error": "Failed to seed Bashkit skill file", "details": e.to_string()}),
        )?;
    }

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(())
}

async fn seed_function_catalogue(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    bash: &Bash,
) -> Result<(), serde_json::Value> {
    let catalogue = crate::builtin_tools::monty::function_catalogue_for_conversation(
        pool,
        sub,
        conversation_id,
    )
    .await
    .map_err(|e| json!({"error": "Failed to get function catalogue", "details": e}))?;

    let fs = bash.fs();
    fs.mkdir(Path::new(FUNCTIONS_DIR), true).await.map_err(
        |e| json!({"error": "Failed to seed Bashkit functions directory", "details": e.to_string()}),
    )?;

    for file in catalogue.files {
        write_vfs_file(fs.as_ref(), &file.path, &file.contents).await?;
    }

    Ok(())
}

async fn seed_datasets(
    pool: &Pool,
    sub: &str,
    prompt_id: i32,
    _conversation_id: i64,
    bash: &Bash,
) -> Result<(), serde_json::Value> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let fs = bash.fs();
    fs.mkdir(Path::new(DATASETS_DIR), true)
        .await
        .map_err(|e| json!({"error": "Failed to seed Bashkit VFS", "details": e.to_string()}))?;

    let datasets = queries::prompts::prompt_datasets()
        .bind(&transaction, &prompt_id)
        .all()
        .await
        .map_err(|e| json!({"error": "Failed to get datasets", "details": e.to_string()}))?;

    let mut dataset_entries = Vec::new();
    for dataset in datasets {
        let dataset_path = format!("{}/{}", DATASETS_DIR, dataset.dataset_id);
        fs.mkdir(Path::new(&dataset_path), true).await.map_err(
            |e| json!({"error": "Failed to seed dataset directory", "details": e.to_string()}),
        )?;

        dataset_entries.push(DatasetEntry {
            dataset_id: dataset.dataset_id,
            name: dataset.name.clone(),
            path: dataset_path.clone(),
        });

        write_json(
            fs.as_ref(),
            &format!("{dataset_path}/metadata.json"),
            &DatasetMetadata {
                dataset_id: dataset.dataset_id,
                name: dataset.name.clone(),
            },
        )
        .await?;

        let documents = dataset_documents(&transaction, prompt_id, dataset.dataset_id).await?;

        let mut file_entries = Vec::new();
        for document in documents {
            let file_path = format!("{dataset_path}/files/{}", document.id);
            let chunks_path = format!("{file_path}/chunks");
            fs.mkdir(Path::new(&chunks_path), true).await.map_err(
                |e| json!({"error": "Failed to seed document directory", "details": e.to_string()}),
            )?;

            file_entries.push(FileEntry {
                document_id: document.id,
                name: document.file_name.clone(),
                size: document.content_size,
                chunks: document.chunk_count,
                path: file_path.clone(),
            });

            write_json(
                fs.as_ref(),
                &format!("{file_path}/metadata.json"),
                &FileMetadata {
                    document_id: document.id,
                    dataset_id: dataset.dataset_id,
                    name: document.file_name,
                    size: document.content_size,
                    chunks: document.chunk_count,
                },
            )
            .await?;

            let chunks =
                document_chunks(&transaction, prompt_id, dataset.dataset_id, document.id).await?;

            for chunk in chunks {
                let path = format!("{chunks_path}/{}.txt", chunk.id);
                fs.write_file(Path::new(&path), chunk.text.as_bytes())
                    .await
                    .map_err(
                        |e| json!({"error": "Failed to seed chunk file", "details": e.to_string()}),
                    )?;
            }
        }

        write_json(
            fs.as_ref(),
            &format!("{dataset_path}/files.json"),
            &DatasetFiles {
                files: file_entries,
            },
        )
        .await?;
    }

    write_json(
        fs.as_ref(),
        &format!("{DATASETS_DIR}/index.json"),
        &DatasetManifest {
            datasets: dataset_entries,
        },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(())
}

async fn write_json<T: Serialize + ?Sized>(
    fs: &dyn FileSystem,
    path: &str,
    value: &T,
) -> Result<(), serde_json::Value> {
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|e| json!({"error": "Failed to serialize VFS JSON", "details": e.to_string()}))?;
    fs.write_file(Path::new(path), &bytes)
        .await
        .map_err(|e| json!({"error": "Failed to write Bashkit VFS file", "details": e.to_string()}))
}

async fn seed_attachments(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    bash: &Bash,
) -> Result<(), serde_json::Value> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let fs = bash.fs();
    fs.mkdir(Path::new(ATTACHMENTS_DIR), true).await.map_err(
        |e| json!({"error": "Failed to seed attachments directory", "details": e.to_string()}),
    )?;

    let attachments = queries::attachments::get_by_conversation()
        .bind(&transaction, &conversation_id)
        .all()
        .await
        .map_err(|e| json!({"error": "Failed to get attachments", "details": e.to_string()}))?;

    let planned_paths = plan_attachment_paths(
        attachments
            .iter()
            .map(|attachment| attachment.file_name.as_str()),
    );
    let mut entries = Vec::new();

    for (attachment, path) in attachments.iter().zip(planned_paths) {
        let data = queries::attachments::get_content()
            .bind(&transaction, &attachment.id)
            .one()
            .await
            .map_err(|e| {
                json!({
                    "error": "Failed to get attachment content",
                    "details": e.to_string()
                })
            })?;

        fs.write_file(Path::new(&path), &data.object_data)
            .await
            .map_err(
                |e| json!({"error": "Failed to seed attachment file", "details": e.to_string()}),
            )?;

        entries.push(AttachmentEntry {
            file_id: attachment.id,
            name: attachment.file_name.clone(),
            path,
            mime_type: attachment.mime_type.clone(),
            size: attachment.file_size,
        });
    }

    write_json(
        fs.as_ref(),
        &format!("{ATTACHMENTS_DIR}/index.json"),
        &AttachmentManifest { files: entries },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(())
}

async fn seed_outputs(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    bash: &Bash,
) -> Result<(), serde_json::Value> {
    let conversation_id = db_conversation_id(conversation_id)?;
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let fs = bash.fs();
    fs.mkdir(Path::new(OUTPUT_DIR), true).await.map_err(
        |e| json!({"error": "Failed to seed output directory", "details": e.to_string()}),
    )?;

    let outputs = queries::generated_outputs::list_by_conversation()
        .bind(&transaction, &conversation_id)
        .all()
        .await
        .map_err(
            |e| json!({"error": "Failed to get generated outputs", "details": e.to_string()}),
        )?;

    for output in outputs {
        if !is_output_path(&output.path) {
            continue;
        }

        let data = queries::generated_outputs::get_content()
            .bind(&transaction, &output.id)
            .one()
            .await
            .map_err(|e| {
                json!({
                    "error": "Failed to get generated output content",
                    "details": e.to_string()
                })
            })?;

        write_vfs_file(fs.as_ref(), &output.path, &data.object_data).await?;
    }

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(())
}

async fn persist_outputs(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    bash: &Bash,
) -> Result<Vec<OutputEntry>, serde_json::Value> {
    let conversation_id_i32 = db_conversation_id(conversation_id)?;
    let fs = bash.fs();
    fs.mkdir(Path::new(OUTPUT_DIR), true).await.map_err(
        |e| json!({"error": "Failed to inspect output directory", "details": e.to_string()}),
    )?;

    let files = collect_output_files(fs.as_ref(), Path::new(OUTPUT_DIR)).await?;
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let (user_id, team_id) = conversation_owner_and_team_id(&transaction, conversation_id).await?;
    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    let storage_config = StorageConfig::database(pool.clone());
    let mut persisted = Vec::new();

    for file in files {
        let bytes = fs.read_file(Path::new(&file)).await.map_err(
            |e| json!({"error": "Failed to read output file", "path": file, "details": e.to_string()}),
        )?;
        if bytes.is_empty() {
            continue;
        }
        let hash = format!("{:x}", md5::compute(&bytes));
        let file_name = output_file_name(&file);
        let mime_type = output_mime_type(&file_name);
        let file_size = bytes.len() as i64;

        let existing = existing_output(pool, sub, conversation_id, &file).await?;
        if existing
            .as_ref()
            .is_some_and(|existing| existing.file_hash == hash)
        {
            continue;
        }

        let object_id =
            object_storage::upload(&storage_config, user_id, team_id, &file_name, &bytes)
                .await
                .map_err(|e| {
                    json!({
                        "error": "Failed to store output file",
                        "path": file,
                        "details": e.to_string()
                    })
                })?;

        let id = upsert_output(
            pool,
            sub,
            conversation_id_i32,
            object_id,
            &file,
            &file_name,
            &mime_type,
            file_size,
            &hash,
        )
        .await?;

        persisted.push(OutputEntry {
            id,
            path: file,
            file_name,
            mime_type,
            size: file_size,
        });
    }

    Ok(persisted)
}

async fn collect_output_files(
    fs: &dyn FileSystem,
    root: &Path,
) -> Result<Vec<String>, serde_json::Value> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();

    while let Some(dir) = pending.pop() {
        for entry in fs.read_dir(&dir).await.map_err(
            |e| json!({"error": "Failed to read output directory", "path": dir.display().to_string(), "details": e.to_string()}),
        )? {
            let path = dir.join(&entry.name);
            if entry.metadata.file_type == FileType::Directory {
                pending.push(path);
            } else if entry.metadata.file_type == FileType::File
                && entry.metadata.size <= MAX_OUTPUT_FILE_BYTES
            {
                let path = path.to_string_lossy().to_string();
                if is_output_path(&path) {
                    files.push(path);
                    if files.len() >= MAX_OUTPUT_FILES {
                        files.sort();
                        return Ok(files);
                    }
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

async fn existing_output(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    path: &str,
) -> Result<Option<db::GeneratedOutput>, serde_json::Value> {
    let conversation_id = db_conversation_id(conversation_id)?;
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let output = queries::generated_outputs::find_by_path()
        .bind(&transaction, &conversation_id, &path)
        .opt()
        .await
        .map_err(
            |e| json!({"error": "Failed to inspect output metadata", "details": e.to_string()}),
        )?;

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(output)
}

#[allow(clippy::too_many_arguments)]
async fn upsert_output(
    pool: &Pool,
    sub: &str,
    conversation_id: i32,
    object_id: i32,
    path: &str,
    file_name: &str,
    mime_type: &str,
    file_size: i64,
    file_hash: &str,
) -> Result<i32, serde_json::Value> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let id = queries::generated_outputs::upsert()
        .bind(
            &transaction,
            &conversation_id,
            &object_id,
            &path,
            &file_name,
            &mime_type,
            &file_size,
            &file_hash,
        )
        .one()
        .await
        .map_err(
            |e| json!({"error": "Failed to persist output metadata", "details": e.to_string()}),
        )?;

    transaction
        .commit()
        .await
        .map_err(|e| json!({"error": "Failed to commit transaction", "details": e.to_string()}))?;

    Ok(id)
}

async fn conversation_owner_and_team_id(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<(i32, i32), serde_json::Value> {
    let row = transaction
        .query_one(
            "SELECT user_id, team_id FROM llm.conversations WHERE id = $1 AND user_id = current_app_user()",
            &[&conversation_id],
        )
        .await
        .map_err(|e| json!({"error": "Failed to get conversation", "details": e.to_string()}))?;
    Ok((row.get(0), row.get(1)))
}

async fn write_vfs_file(
    fs: &dyn FileSystem,
    path: &str,
    contents: &[u8],
) -> Result<(), serde_json::Value> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs.mkdir(parent, true).await.map_err(
            |e| json!({"error": "Failed to create VFS directory", "details": e.to_string()}),
        )?;
    }
    fs.write_file(path, contents)
        .await
        .map_err(|e| json!({"error": "Failed to write VFS file", "details": e.to_string()}))
}

fn is_output_path(path: &str) -> bool {
    path == OUTPUT_DIR || path.starts_with(&format!("{OUTPUT_DIR}/"))
}

fn output_file_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("output")
        .chars()
        .take(255)
        .collect()
}

fn output_mime_type(file_name: &str) -> String {
    mime_guess::from_path(file_name)
        .first_or_octet_stream()
        .to_string()
        .chars()
        .take(50)
        .collect()
}

fn db_conversation_id(conversation_id: i64) -> Result<i32, serde_json::Value> {
    i32::try_from(conversation_id)
        .map_err(|_| json!({"error": "conversation_id is outside the supported range"}))
}

fn plan_attachment_paths<'a>(file_names: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let mut used = HashSet::new();
    file_names
        .into_iter()
        .map(|file_name| {
            let safe_name = sanitize_attachment_file_name(file_name);
            let unique_name = unique_attachment_file_name(&safe_name, &mut used);
            format!("{ATTACHMENTS_DIR}/{unique_name}")
        })
        .collect()
}

fn sanitize_attachment_file_name(file_name: &str) -> String {
    let leaf = file_name
        .rsplit(['/', '\\'])
        .find(|part| !part.is_empty())
        .unwrap_or(file_name);
    let sanitized = leaf
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_' | ' ') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches(['.', ' '])
        .to_string();

    if sanitized.is_empty() {
        "attachment".to_string()
    } else {
        sanitized
    }
}

fn unique_attachment_file_name(file_name: &str, used: &mut HashSet<String>) -> String {
    if used.insert(file_name.to_string()) {
        return file_name.to_string();
    }

    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or(file_name);
    let extension = path.extension().and_then(|extension| extension.to_str());

    for suffix in 2.. {
        let candidate = match extension {
            Some(extension) if !extension.is_empty() => format!("{stem}-{suffix}.{extension}"),
            _ => format!("{stem}-{suffix}"),
        };
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }

    unreachable!("unbounded suffix loop should always return")
}

async fn conversation_team_id(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<i32, serde_json::Value> {
    let row = transaction
        .query_one(
            "SELECT team_id FROM llm.conversations WHERE id = $1",
            &[&conversation_id],
        )
        .await
        .map_err(|e| json!({"error": "Failed to get conversation", "details": e.to_string()}))?;
    Ok(row.get(0))
}

struct SeedDocument {
    id: i32,
    file_name: String,
    content_size: i32,
    chunk_count: i64,
}

struct SeedChunk {
    id: i32,
    text: String,
}

async fn dataset_documents(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    dataset_id: i32,
) -> Result<Vec<SeedDocument>, serde_json::Value> {
    let rows = transaction
        .query(
            "
            SELECT
                d.id,
                d.file_name,
                d.content_size,
                (SELECT COUNT(id) FROM rag.chunks WHERE document_id = d.id) AS chunk_count
            FROM rag.documents d
            WHERE d.dataset_id = $1
              AND d.dataset_id IN (
                  SELECT dataset_id FROM assistants.prompt_dataset WHERE prompt_id = $2
              )
            ORDER BY d.updated_at DESC
            ",
            &[&dataset_id, &prompt_id],
        )
        .await
        .map_err(|e| json!({"error": "Failed to get dataset files", "details": e.to_string()}))?;

    Ok(rows
        .into_iter()
        .map(|row| SeedDocument {
            id: row.get(0),
            file_name: row.get(1),
            content_size: row.get(2),
            chunk_count: row.get(3),
        })
        .collect())
}

async fn document_chunks(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    dataset_id: i32,
    document_id: i32,
) -> Result<Vec<SeedChunk>, serde_json::Value> {
    let rows = transaction
        .query(
            "
            SELECT c.id, decrypt_text(c.text) AS text
            FROM rag.chunks c
            INNER JOIN rag.documents d ON d.id = c.document_id
            WHERE c.document_id = $1
              AND d.dataset_id = $2
              AND d.dataset_id IN (
                  SELECT dataset_id FROM assistants.prompt_dataset WHERE prompt_id = $3
              )
            ORDER BY c.page_number ASC, c.id ASC
            LIMIT $4
            ",
            &[
                &document_id,
                &dataset_id,
                &prompt_id,
                &CHUNKS_PER_DOCUMENT_LIMIT,
            ],
        )
        .await
        .map_err(|e| json!({"error": "Failed to get document chunks", "details": e.to_string()}))?;

    Ok(rows
        .into_iter()
        .map(|row| SeedChunk {
            id: row.get(0),
            text: row.get(1),
        })
        .collect())
}

struct RagReadBuiltin;

#[async_trait]
impl Builtin for RagReadBuiltin {
    async fn execute(&self, ctx: BuiltinContext<'_>) -> bashkit::Result<ExecResult> {
        let Some(path) = ctx.args.first() else {
            return Ok(ExecResult::err(
                "usage: rag-read /home/user/datasets/.../chunks/<id>.txt\n",
                2,
            ));
        };

        if parse_chunk_path(path).is_none() {
            return Ok(ExecResult::err(
                "rag-read only accepts /home/user/datasets/{dataset_id}/files/{document_id}/chunks/{chunk_id}.txt\n",
                2,
            ));
        }

        match ctx.fs.read_file(Path::new(path)).await {
            Ok(bytes) => Ok(ExecResult::ok(String::from_utf8_lossy(&bytes).to_string())),
            Err(err) => Ok(ExecResult::err(format!("{err}\n"), 1)),
        }
    }

    fn llm_hint(&self) -> Option<&'static str> {
        Some("rag-read PATH: read a dataset chunk file from /home/user/datasets after rag-search returns paths.")
    }
}

struct RagSearchBuiltin {
    pool: Pool,
    sub: String,
    conversation_id: i64,
    prompt_id: i32,
}

#[async_trait]
impl Builtin for RagSearchBuiltin {
    async fn execute(&self, ctx: BuiltinContext<'_>) -> bashkit::Result<ExecResult> {
        let (query, limit) = parse_rag_search_args(ctx.args);
        if query.trim().is_empty() {
            return Ok(ExecResult::err("usage: rag-search QUERY [--limit N]\n", 2));
        }

        match execute_rag_search(
            &self.pool,
            &self.sub,
            self.conversation_id,
            self.prompt_id,
            &query,
            limit,
        )
        .await
        {
            Ok(value) => Ok(ExecResult::ok(format!("{value}\n"))),
            Err(err) => Ok(ExecResult::err(format!("{err}\n"), 1)),
        }
    }

    fn llm_hint(&self) -> Option<&'static str> {
        Some("rag-search QUERY [--limit N]: search assistant datasets and return matching chunk paths as JSON.")
    }
}

fn parse_rag_search_args(args: &[String]) -> (String, i32) {
    let mut limit = 5;
    let mut query_parts = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--limit" {
            if let Some(value) = iter.next().and_then(|value| value.parse::<i32>().ok()) {
                limit = value.clamp(1, 20);
            }
        } else {
            query_parts.push(arg.as_str());
        }
    }
    (query_parts.join(" "), limit)
}

async fn execute_rag_search(
    pool: &Pool,
    sub: &str,
    conversation_id: i64,
    prompt_id: i32,
    query: &str,
    limit: i32,
) -> Result<Value, serde_json::Value> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| json!({"error": "Failed to get DB client", "details": e.to_string()}))?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| json!({"error": "Failed to start transaction", "details": e.to_string()}))?;

    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|e| json!({"error": "Failed to set RLS", "details": e.to_string()}))?;

    let chunks = search_context(&transaction, prompt_id, conversation_id, query, limit).await;

    if chunks.is_ok() {
        transaction
            .commit()
            .await
            .map_err(|e| json!({"error": "Failed to commit", "details": e.to_string()}))?;
    } else {
        transaction.rollback().await.ok();
    }

    chunks
}

async fn search_context(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    conversation_id: i64,
    query: &str,
    limit: i32,
) -> Result<Value, serde_json::Value> {
    let team_id = conversation_team_id(transaction, conversation_id).await?;
    let prompt = queries::prompts::prompt()
        .bind(transaction, &prompt_id, &team_id)
        .one()
        .await
        .map_err(|e| json!({"error": "Failed to fetch prompt", "details": e.to_string()}))?;

    let (base_url, model, api_key) = match (
        prompt.embeddings_base_url,
        prompt.embeddings_model,
        prompt.embeddings_api_key,
    ) {
        (Some(url), Some(model), api_key) => (url, model, api_key),
        _ => return Err(json!({"error": "Prompt missing embeddings configuration"})),
    };

    let embeddings = get_embeddings_via_rig(
        query,
        &base_url,
        &model,
        prompt.embeddings_context_size.unwrap_or(256),
        api_key.as_deref(),
    )
    .await
    .map_err(|e| json!({"error": "Failed to get embeddings", "details": e}))?;

    let related = db::get_related_context(transaction, prompt_id, limit, embeddings)
        .await
        .map_err(|e| json!({"error": "Failed to search context", "details": e.to_string()}))?;

    let chunk_ids: Vec<i32> = related.iter().map(|chunk| chunk.chunk_id).collect();
    let paths = chunk_paths(transaction, prompt_id, &chunk_ids).await?;
    let paths_by_chunk: HashMap<i32, ChunkPath> = paths
        .into_iter()
        .map(|path| (path.chunk_id, path))
        .collect();

    let mut chunks = Vec::new();
    for chunk in related {
        queries::chats_chunks::create_chunks_chats()
            .bind(transaction, &chunk.chunk_id, &conversation_id)
            .await
            .map_err(
                |e| json!({"error": "Failed to record chunk usage", "details": e.to_string()}),
            )?;

        if let Some(path) = paths_by_chunk.get(&chunk.chunk_id) {
            chunks.push(json!({
                "chunk_id": chunk.chunk_id,
                "path": path.vfs_path(),
                "preview": chunk.chunk_text.chars().take(300).collect::<String>()
            }));
        }
    }

    Ok(json!({"chunks": chunks}))
}

async fn chunk_paths(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    chunk_ids: &[i32],
) -> Result<Vec<ChunkPath>, serde_json::Value> {
    if chunk_ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = transaction
        .query(
            "
            SELECT c.id, c.document_id, d.dataset_id
            FROM rag.chunks c
            INNER JOIN rag.documents d ON d.id = c.document_id
            WHERE c.id = ANY($1)
              AND d.dataset_id IN (
                  SELECT dataset_id FROM assistants.prompt_dataset WHERE prompt_id = $2
              )
            ",
            &[&chunk_ids, &prompt_id],
        )
        .await
        .map_err(|e| json!({"error": "Failed to resolve chunk paths", "details": e.to_string()}))?;

    Ok(rows
        .into_iter()
        .map(|row| ChunkPath {
            chunk_id: row.get(0),
            document_id: row.get(1),
            dataset_id: row.get(2),
        })
        .collect())
}

fn parse_chunk_path(path: &str) -> Option<ChunkPath> {
    let path = PathBuf::from(path);
    let parts: Vec<_> = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect();

    if parts.len() != 9
        || parts[0] != "/"
        || parts[1] != "home"
        || parts[2] != "user"
        || parts[3] != "datasets"
        || parts[5] != "files"
        || parts[7] != "chunks"
    {
        return None;
    }

    let dataset_id = parts[4].parse().ok()?;
    let document_id = parts[6].parse().ok()?;
    let chunk_id = parts[8].strip_suffix(".txt")?.parse().ok()?;

    Some(ChunkPath {
        dataset_id,
        document_id,
        chunk_id,
    })
}

fn trim_to_context_length(input: &str, context_length: i32) -> String {
    if input.is_empty() {
        return String::new();
    }
    let effective_context_length = if context_length <= 0 {
        256
    } else {
        context_length
    };
    let char_count = input.chars().count() as i32;
    if char_count <= effective_context_length {
        return input.to_string();
    }
    input
        .chars()
        .take(effective_context_length as usize)
        .collect()
}

async fn get_embeddings_via_rig(
    input: &str,
    api_end_point: &str,
    model: &str,
    context_length: i32,
    api_key: Option<&str>,
) -> Result<Vec<f32>, String> {
    let text = String::from_utf8_lossy(input.as_bytes()).to_string();
    let trimmed_text = trim_to_context_length(&text, context_length);

    let normalized_base_url = api_end_point
        .strip_suffix("/embeddings")
        .or_else(|| api_end_point.strip_suffix("/v1/embeddings"))
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| api_end_point.trim_end_matches('/').to_string());

    let embedding = if let Some(key) = api_key.filter(|k| !k.trim().is_empty()) {
        let client = openai::Client::builder()
            .api_key(key)
            .base_url(&normalized_base_url)
            .build()
            .map_err(|e| e.to_string())?;
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await
            .map_err(|e| e.to_string())?
    } else {
        let client = ollama::Client::builder()
            .api_key("")
            .base_url(&normalized_base_url)
            .build()
            .map_err(|e| e.to_string())?;
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await
            .map_err(|e| e.to_string())?
    };

    Ok(embedding.vec.into_iter().map(|v| v as f32).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tool_definition() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "run_bash");
        assert!(tool.description.contains("/home/user/attachments"));
        assert!(tool.description.contains("/home/user/functions"));
    }

    #[test]
    fn test_preview_vfs_tree_includes_visible_skill_paths() {
        let preview = preview_vfs_tree(
            &[db::queries::skills::SkillSummary {
                skill_id: 42,
                skill_name: "Presentation Builder".to_string(),
                description: "Build slides".to_string(),
                is_system: true,
            }],
            &[crate::builtin_tools::monty::RuntimeFunctionFile {
                path: "/home/user/functions/email.md".to_string(),
                contents: b"# Email".to_vec(),
            }],
        );

        assert!(preview.contains("/home/user"));
        assert!(preview.contains("`-- skills"));
        assert!(preview.contains("|-- functions"));
        assert!(preview.contains("email.md"));
        assert!(preview.contains("presentation-builder"));
        assert!(preview.contains("SKILL.md"));
        assert!(preview.contains("<uploaded_file_name>"));
        assert!(preview.contains("<chunk_id>.txt"));
        assert!(preview.contains("Omitted from this preview"));
    }

    #[test]
    fn test_sanitize_attachment_file_name() {
        assert_eq!(sanitize_attachment_file_name("report.txt"), "report.txt");
        assert_eq!(
            sanitize_attachment_file_name("../secrets/report?.txt"),
            "report_.txt"
        );
        assert_eq!(
            sanitize_attachment_file_name(r"..\windows\notes.md"),
            "notes.md"
        );
        assert_eq!(sanitize_attachment_file_name("..."), "attachment");
    }

    #[test]
    fn test_plan_attachment_paths_deduplicates_names() {
        let paths = plan_attachment_paths(["report.txt", "report.txt", "report-2.txt", ""]);
        assert_eq!(
            paths,
            vec![
                "/home/user/attachments/report.txt",
                "/home/user/attachments/report-2.txt",
                "/home/user/attachments/report-2-2.txt",
                "/home/user/attachments/attachment",
            ]
        );
    }

    #[test]
    fn test_parse_rag_search_args() {
        let args = vec![
            "quarterly".to_string(),
            "sales".to_string(),
            "--limit".to_string(),
            "7".to_string(),
        ];
        let (query, limit) = parse_rag_search_args(&args);
        assert_eq!(query, "quarterly sales");
        assert_eq!(limit, 7);
    }

    #[test]
    fn test_parse_chunk_path() {
        let path = parse_chunk_path("/home/user/datasets/1/files/2/chunks/3.txt").unwrap();
        assert_eq!(path.dataset_id, 1);
        assert_eq!(path.document_id, 2);
        assert_eq!(path.chunk_id, 3);
    }

    #[test]
    fn test_parse_chunk_path_rejects_other_paths() {
        assert!(parse_chunk_path("/tmp/3.txt").is_none());
        assert!(parse_chunk_path("/datasets/1/files/2/chunks/3.txt").is_none());
        assert!(parse_chunk_path("/home/user/datasets/1/files/2/metadata.json").is_none());
    }
}
