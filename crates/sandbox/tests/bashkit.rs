use async_trait::async_trait;
use bashkit::{FileSystem, InMemoryFs};
use sandbox::{
    BashkitSandbox, Command, DirectoryEntry, FileMetadata, FileType, FilesystemError,
    FilesystemErrorKind, HttpRequest, HttpResponse, NetworkError, PythonCallArguments,
    PythonFunction, PythonFunctionError, RunRequest, Sandbox, SandboxFilesystem, SandboxNetwork,
    WriteMode,
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

struct TestFilesystem(Arc<InMemoryFs>);

fn error(error: bashkit::Error) -> FilesystemError {
    FilesystemError::new(FilesystemErrorKind::Other, error.to_string())
}

fn metadata(value: bashkit::Metadata) -> FileMetadata {
    FileMetadata {
        file_type: match value.file_type {
            bashkit::FileType::File => FileType::File,
            bashkit::FileType::Directory => FileType::Directory,
            bashkit::FileType::Symlink => FileType::Symlink,
            bashkit::FileType::Fifo => FileType::Fifo,
        },
        size: value.size,
        mode: value.mode,
        modified: value.modified,
        created: value.created,
    }
}

#[async_trait]
impl SandboxFilesystem for TestFilesystem {
    async fn stat(&self, path: &Path) -> Result<FileMetadata, FilesystemError> {
        self.0.stat(path).await.map(metadata).map_err(error)
    }
    async fn read_dir(&self, path: &Path) -> Result<Vec<DirectoryEntry>, FilesystemError> {
        self.0
            .read_dir(path)
            .await
            .map(|values| {
                values
                    .into_iter()
                    .map(|value| DirectoryEntry {
                        name: value.name,
                        metadata: metadata(value.metadata),
                    })
                    .collect()
            })
            .map_err(error)
    }
    async fn read(&self, path: &Path) -> Result<Vec<u8>, FilesystemError> {
        self.0.read_file(path).await.map_err(error)
    }
    async fn write(
        &self,
        path: &Path,
        contents: &[u8],
        mode: WriteMode,
    ) -> Result<(), FilesystemError> {
        match mode {
            WriteMode::Truncate => self.0.write_file(path, contents).await,
            WriteMode::Append => self.0.append_file(path, contents).await,
        }
        .map_err(error)
    }
    async fn create_dir(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError> {
        self.0.mkdir(path, recursive).await.map_err(error)
    }
    async fn remove(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError> {
        self.0.remove(path, recursive).await.map_err(error)
    }
    async fn rename(&self, from: &Path, to: &Path) -> Result<(), FilesystemError> {
        self.0.rename(from, to).await.map_err(error)
    }
    async fn copy(&self, from: &Path, to: &Path) -> Result<(), FilesystemError> {
        self.0.copy(from, to).await.map_err(error)
    }
    async fn symlink(&self, target: &Path, link: &Path) -> Result<(), FilesystemError> {
        self.0.symlink(target, link).await.map_err(error)
    }
    async fn read_link(&self, path: &Path) -> Result<PathBuf, FilesystemError> {
        self.0.read_link(path).await.map_err(error)
    }
    async fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), FilesystemError> {
        self.0.chmod(path, mode).await.map_err(error)
    }
    async fn set_modified(&self, path: &Path, modified: SystemTime) -> Result<(), FilesystemError> {
        self.0
            .set_modified_time(path, modified)
            .await
            .map_err(error)
    }
}

#[derive(Default)]
struct RecordingNetwork(Mutex<Vec<String>>);

struct AddFunction;

#[async_trait]
impl PythonFunction for AddFunction {
    fn name(&self) -> &str {
        "connector_add"
    }

    async fn call(&self, arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
        let left = arguments
            .keyword
            .get("left")
            .and_then(Value::as_i64)
            .unwrap();
        let right = arguments
            .keyword
            .get("right")
            .and_then(Value::as_i64)
            .unwrap();
        Ok(json!({"total": left + right}))
    }
}

#[async_trait]
impl SandboxNetwork for RecordingNetwork {
    async fn request(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError> {
        self.0.lock().unwrap().push(request.url);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: b"mediated".to_vec(),
        })
    }
}

fn request(
    command: Command,
    fs: Arc<TestFilesystem>,
    network: Arc<RecordingNetwork>,
) -> RunRequest {
    RunRequest {
        command,
        filesystem: fs,
        network,
        python_functions: Vec::new(),
    }
}

#[tokio::test]
async fn shell_writes_through_supplied_filesystem() {
    let inner = Arc::new(InMemoryFs::new());
    let fs = Arc::new(TestFilesystem(inner.clone()));
    let result = BashkitSandbox
        .run(request(
            Command::Shell {
                script: "mkdir -p /home/user/work && printf new > /home/user/work/example.txt"
                    .into(),
                timeout: Duration::from_secs(5),
            },
            fs,
            Arc::new(RecordingNetwork::default()),
        ))
        .await
        .unwrap();
    assert_eq!(result.execution.exit_code, 0);
    assert_eq!(
        inner
            .read_file(Path::new("/home/user/work/example.txt"))
            .await
            .unwrap(),
        b"new"
    );
}

#[tokio::test]
async fn python_open_uses_supplied_filesystem() {
    let inner = Arc::new(InMemoryFs::new());
    inner
        .mkdir(Path::new("/home/user/work"), true)
        .await
        .unwrap();
    inner
        .write_file(Path::new("/home/user/work/input.txt"), b"hello")
        .await
        .unwrap();
    let result = BashkitSandbox
        .run(request(
            Command::Python {
                code: "print(open('/home/user/work/input.txt').read())".into(),
                timeout: Duration::from_secs(5),
            },
            Arc::new(TestFilesystem(inner)),
            Arc::new(RecordingNetwork::default()),
        ))
        .await
        .unwrap();
    assert_eq!(result.execution.stdout, "hello\n");
}

#[tokio::test]
async fn python_can_call_scoped_host_functions_in_a_loop() {
    let mut run_request = request(
        Command::Python {
            code:
                "for value in [1, 2, 3]:\n    print(connector_add(left=value, right=10)['total'])"
                    .into(),
            timeout: Duration::from_secs(5),
        },
        Arc::new(TestFilesystem(Arc::new(InMemoryFs::new()))),
        Arc::new(RecordingNetwork::default()),
    );
    run_request.python_functions = vec![Arc::new(AddFunction)];
    let result = BashkitSandbox.run(run_request).await.unwrap();
    assert_eq!(result.execution.exit_code, 0);
    assert_eq!(result.execution.stdout, "11\n12\n13\n");
}

#[tokio::test]
async fn curl_uses_supplied_network() {
    let network = Arc::new(RecordingNetwork::default());
    let result = BashkitSandbox
        .run(request(
            Command::Shell {
                script: "curl -s https://example.com/data".into(),
                timeout: Duration::from_secs(5),
            },
            Arc::new(TestFilesystem(Arc::new(InMemoryFs::new()))),
            network.clone(),
        ))
        .await
        .unwrap();
    assert_eq!(result.execution.stdout, "mediated");
    assert_eq!(
        network.0.lock().unwrap().as_slice(),
        ["https://example.com/data"]
    );
}
