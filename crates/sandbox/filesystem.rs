use async_trait::async_trait;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Fifo,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub file_type: FileType,
    pub size: u64,
    pub mode: u32,
    pub modified: SystemTime,
    pub created: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub name: String,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    Truncate,
    Append,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemErrorKind {
    NotFound,
    AlreadyExists,
    PermissionDenied,
    InvalidInput,
    NotDirectory,
    IsDirectory,
    DirectoryNotEmpty,
    Unsupported,
    ResourceExhausted,
    Other,
}

#[derive(Debug, Clone)]
pub struct FilesystemError {
    pub kind: FilesystemErrorKind,
    pub message: String,
}

impl FilesystemError {
    pub fn new(kind: FilesystemErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for FilesystemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for FilesystemError {}

#[async_trait]
pub trait SandboxFilesystem: Send + Sync {
    async fn stat(&self, path: &Path) -> Result<FileMetadata, FilesystemError>;
    async fn read_dir(&self, path: &Path) -> Result<Vec<DirectoryEntry>, FilesystemError>;
    async fn read(&self, path: &Path) -> Result<Vec<u8>, FilesystemError>;
    async fn write(
        &self,
        path: &Path,
        contents: &[u8],
        mode: WriteMode,
    ) -> Result<(), FilesystemError>;
    async fn create_dir(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError>;
    async fn remove(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError>;
    async fn rename(&self, from: &Path, to: &Path) -> Result<(), FilesystemError>;
    async fn copy(&self, from: &Path, to: &Path) -> Result<(), FilesystemError>;
    async fn symlink(&self, target: &Path, link: &Path) -> Result<(), FilesystemError>;
    async fn read_link(&self, path: &Path) -> Result<PathBuf, FilesystemError>;
    async fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), FilesystemError>;
    async fn set_modified(&self, path: &Path, modified: SystemTime) -> Result<(), FilesystemError>;

    async fn exists(&self, path: &Path) -> Result<bool, FilesystemError> {
        match self.stat(path).await {
            Ok(_) => Ok(true),
            Err(error) if error.kind == FilesystemErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }
}
