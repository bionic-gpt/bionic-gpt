//! One isolated upstream stdio MCP process per drawing. No CAD commands are reimplemented here.
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::Mutex,
};
use uuid::Uuid;

const MAX_REPLY: u64 = 6 * 1024 * 1024;
#[cfg(not(test))]
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
#[cfg(test)]
const REQUEST_TIMEOUT: Duration = Duration::from_millis(50);
const TTL: Duration = Duration::from_secs(24 * 60 * 60);
const MAX_DRAWINGS: usize = 16;
pub const TOOLS: &[&str] = &[
    "command_line",
    "script",
    "execute",
    "list_commands",
    "inspect_drawing",
    "query_entities",
    "render",
    "cancel",
];

pub struct Backend {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    sequence: u64,
    broken: bool,
    _directory: tempfile::TempDir,
}

impl Backend {
    pub async fn start() -> Result<Self, String> {
        let directory = tempfile::tempdir().map_err(|e| e.to_string())?;
        let mut child = Command::new(std::env::current_exe().map_err(|e| e.to_string())?)
            .arg("--worker")
            .arg(directory.path())
            .arg(std::env::var("CADCRAFT_CLI").unwrap_or_else(|_| "/app/cadcraft-cli".into()))
            .env_clear()
            .env("CADCRAFT_CONFIG_DIR", "")
            .current_dir(directory.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| e.to_string())?;
        let input = child.stdin.take().ok_or("Missing worker stdin")?;
        let output = BufReader::new(child.stdout.take().ok_or("Missing worker stdout")?);
        let mut backend = Self {
            child,
            input,
            output,
            sequence: 0,
            broken: false,
            _directory: directory,
        };
        backend.request("initialize", json!({"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"bionic-cadcraft","version":"0.2.0"}})).await?;
        backend
            .input
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
            .await
            .map_err(|e| e.to_string())?;
        Ok(backend)
    }

    pub async fn request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        if self.broken {
            return Err("Drawing session unavailable; open a new drawing".into());
        }
        self.sequence += 1;
        let id = self.sequence;
        let operation = async {
            let mut bytes = serde_json::to_vec(
                &json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}),
            )
            .map_err(|e| e.to_string())?;
            bytes.push(b'\n');
            self.input
                .write_all(&bytes)
                .await
                .map_err(|e| e.to_string())?;
            self.input.flush().await.map_err(|e| e.to_string())?;
            let mut line = Vec::new();
            (&mut self.output)
                .take(MAX_REPLY + 1)
                .read_until(b'\n', &mut line)
                .await
                .map_err(|e| e.to_string())?;
            if line.len() as u64 > MAX_REPLY || !line.ends_with(b"\n") {
                return Err("Worker closed or exceeded reply limit".into());
            }
            let response: Value = serde_json::from_slice(&line).map_err(|e| e.to_string())?;
            if response["id"] != id {
                return Err("Worker reply ID mismatch".into());
            }
            Ok::<Value, String>(response)
        };
        let response = match tokio::time::timeout(REQUEST_TIMEOUT, operation).await {
            Ok(Ok(response)) => response,
            result => {
                self.broken = true;
                let _ = self.child.kill().await;
                return Err(match result {
                    Err(_) => "Command timed out; it may have run. Session stopped; do not retry automatically".into(),
                    Ok(Err(error)) => format!("{error}; command may have run. Session stopped"),
                    _ => unreachable!(),
                });
            }
        };
        if let Some(error) = response.get("error") {
            return Err(error.to_string());
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| "Missing MCP result".into())
    }

    pub async fn tool(&mut self, name: &str, arguments: Value) -> Result<Value, String> {
        self.request("tools/call", json!({"name":name,"arguments":arguments}))
            .await
    }

    pub async fn snapshot(&mut self) -> Result<String, String> {
        let result = self
            .tool("execute", json!({"command":"document.bytes","params":{}}))
            .await?;
        if result["isError"] == true {
            return Err(format!("Cannot export drawing: {}", result["content"]));
        }
        // Upstream returns engine results as JSON in a text block.
        let text = result["content"]
            .as_array()
            .and_then(|a| a.iter().find_map(|v| v["text"].as_str()))
            .ok_or("Missing drawing export")?;
        let document: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let data = document["data"].as_str().ok_or("Missing DXF data")?;
        if data.len() > 4 * 1024 * 1024 {
            return Err("Drawing exceeds the 3 MiB viewer limit".into());
        }
        Ok(data.to_owned())
    }
}

pub struct Drawing {
    pub backend: Backend,
    pub revision: u64,
    touched: Instant,
}
struct Entry {
    owner: String,
    drawing: Arc<Mutex<Drawing>>,
}
#[derive(Default)]
pub struct Drawings {
    entries: Mutex<HashMap<String, Entry>>,
}
impl Drawings {
    pub async fn open(
        &self,
        owner: &str,
        id: Option<&str>,
    ) -> Result<(String, Arc<Mutex<Drawing>>), String> {
        if let Some(id) = id {
            return Ok((id.to_owned(), self.get(owner, id).await?));
        }
        // Hold the registry lock through creation to enforce the capacity limit.
        let mut entries = self.entries.lock().await;
        if entries.len() >= MAX_DRAWINGS {
            return Err("Drawing capacity reached; try again later".into());
        }
        let backend = Backend::start().await?;
        let id = Uuid::new_v4().to_string();
        let drawing = Arc::new(Mutex::new(Drawing {
            backend,
            revision: 0,
            touched: Instant::now(),
        }));
        entries.insert(
            id.clone(),
            Entry {
                owner: owner.to_owned(),
                drawing: drawing.clone(),
            },
        );
        Ok((id, drawing))
    }
    pub async fn get(&self, owner: &str, id: &str) -> Result<Arc<Mutex<Drawing>>, String> {
        let entries = self.entries.lock().await;
        let entry = entries
            .get(id)
            .filter(|e| e.owner == owner)
            .ok_or("Drawing unavailable in this conversation; open a new drawing")?;
        let selected = entry.drawing.clone();
        drop(entries);
        let mut drawing = selected.lock().await;
        if drawing.touched.elapsed() >= TTL {
            return Err("Drawing expired; open a new drawing".into());
        }
        drawing.touched = Instant::now();
        drop(drawing);
        Ok(selected)
    }
    pub async fn reap(&self) {
        let mut entries = self.entries.lock().await;
        entries.retain(|_, e| match e.drawing.try_lock() {
            Ok(d) => !d.backend.broken && d.touched.elapsed() < TTL,
            Err(_) => true,
        });
    }
}

/// Run only the CLI with filesystem access limited to its workspace and runtime libraries.
/// Fail closed if the kernel cannot enforce Landlock. No server credentials enter the worker.
pub fn worker(directory: PathBuf, cli: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use landlock::{
        ABI, Access, AccessFs, CompatLevel, Compatible, PathBeneath, PathFd, Ruleset, RulesetAttr,
        RulesetCreatedAttr, RulesetStatus,
    };
    use std::os::unix::process::CommandExt;
    let all = AccessFs::from_all(ABI::V3);
    let read = AccessFs::from_read(ABI::V3);
    let mut rules = Ruleset::default()
        .set_compatibility(CompatLevel::HardRequirement)
        .handle_access(all)?
        .create()?;
    rules = rules.add_rule(PathBeneath::new(PathFd::new(&directory)?, all))?;
    rules = rules.add_rule(PathBeneath::new(
        PathFd::new(&cli)?,
        AccessFs::ReadFile | AccessFs::Execute,
    ))?;
    for path in [
        "/usr/lib",
        "/lib",
        "/lib64",
        "/usr/share/fonts",
        "/etc/fonts",
        "/dev/null",
        "/dev/urandom",
    ] {
        if std::path::Path::new(path).exists() {
            let access = if std::path::Path::new(path).is_dir() {
                read
            } else {
                AccessFs::ReadFile | AccessFs::Execute
            };
            rules = rules.add_rule(PathBeneath::new(PathFd::new(path)?, access))?;
        }
    }
    let status = rules.restrict_self()?;
    if status.ruleset != RulesetStatus::FullyEnforced {
        return Err("Landlock ABI v3 is required for drawing isolation".into());
    }
    Err(std::process::Command::new(cli).arg("mcp").exec().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent_backend() -> Backend {
        let directory = tempfile::tempdir().unwrap();
        let mut child = Command::new("sh")
            .args(["-c", "read line; sleep 1"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        Backend {
            input: child.stdin.take().unwrap(),
            output: BufReader::new(child.stdout.take().unwrap()),
            child,
            sequence: 0,
            broken: false,
            _directory: directory,
        }
    }

    #[tokio::test]
    async fn timed_out_commands_stop_the_session_and_are_never_retried() {
        let mut backend = silent_backend();
        let error = backend.request("tools/call", json!({})).await.unwrap_err();
        assert!(error.contains("may have run"));
        assert!(backend.broken);
        assert!(
            backend
                .request("tools/call", json!({}))
                .await
                .unwrap_err()
                .contains("unavailable")
        );
        assert_eq!(backend.sequence, 1);
    }

    #[tokio::test]
    async fn owners_and_expiry_are_checked_before_returning_a_drawing() {
        let registry = Drawings::default();
        let directory;
        let drawing = {
            let backend = silent_backend();
            directory = backend._directory.path().to_owned();
            Arc::new(Mutex::new(Drawing {
                backend,
                revision: 0,
                touched: Instant::now(),
            }))
        };
        registry.entries.lock().await.insert(
            "one".into(),
            Entry {
                owner: "owner:1".into(),
                drawing: drawing.clone(),
            },
        );
        assert!(registry.get("owner:2", "one").await.is_err());
        assert!(registry.get("owner:1", "one").await.is_ok());
        drawing.lock().await.touched = Instant::now() - TTL;
        assert!(registry.get("owner:1", "one").await.is_err());
        drop(drawing);
        registry.reap().await;
        assert!(registry.entries.lock().await.is_empty());
        assert!(!directory.exists());
    }
}
