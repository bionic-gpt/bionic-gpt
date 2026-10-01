use crate::{
    DirectoryEntry, FileMetadata, FileType, FilesystemError, FilesystemErrorKind,
    SandboxFilesystem, WriteMode,
};
use async_trait::async_trait;
use bashkit::{FileSystem, FileSystemExt};
use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

pub(super) struct FilesystemAdapter(pub(super) Arc<dyn SandboxFilesystem>);

impl FileSystemExt for FilesystemAdapter {}

#[async_trait]
impl FileSystem for FilesystemAdapter {
    async fn read_file(&self, path: &Path) -> bashkit::Result<Vec<u8>> {
        self.0.read(path).await.map_err(to_bashkit_error)
    }

    async fn write_file(&self, path: &Path, content: &[u8]) -> bashkit::Result<()> {
        self.0
            .write(path, content, WriteMode::Truncate)
            .await
            .map_err(to_bashkit_error)
    }

    async fn append_file(&self, path: &Path, content: &[u8]) -> bashkit::Result<()> {
        self.0
            .write(path, content, WriteMode::Append)
            .await
            .map_err(to_bashkit_error)
    }

    async fn mkdir(&self, path: &Path, recursive: bool) -> bashkit::Result<()> {
        self.0
            .create_dir(path, recursive)
            .await
            .map_err(to_bashkit_error)
    }

    async fn remove(&self, path: &Path, recursive: bool) -> bashkit::Result<()> {
        self.0
            .remove(path, recursive)
            .await
            .map_err(to_bashkit_error)
    }

    async fn stat(&self, path: &Path) -> bashkit::Result<bashkit::Metadata> {
        self.0
            .stat(path)
            .await
            .map(to_metadata)
            .map_err(to_bashkit_error)
    }

    async fn read_dir(&self, path: &Path) -> bashkit::Result<Vec<bashkit::DirEntry>> {
        self.0
            .read_dir(path)
            .await
            .map(|entries| entries.into_iter().map(to_entry).collect())
            .map_err(to_bashkit_error)
    }

    async fn exists(&self, path: &Path) -> bashkit::Result<bool> {
        self.0.exists(path).await.map_err(to_bashkit_error)
    }

    async fn rename(&self, from: &Path, to: &Path) -> bashkit::Result<()> {
        self.0.rename(from, to).await.map_err(to_bashkit_error)
    }

    async fn copy(&self, from: &Path, to: &Path) -> bashkit::Result<()> {
        self.0.copy(from, to).await.map_err(to_bashkit_error)
    }

    async fn symlink(&self, target: &Path, link: &Path) -> bashkit::Result<()> {
        self.0.symlink(target, link).await.map_err(to_bashkit_error)
    }

    async fn read_link(&self, path: &Path) -> bashkit::Result<PathBuf> {
        self.0.read_link(path).await.map_err(to_bashkit_error)
    }

    async fn chmod(&self, path: &Path, mode: u32) -> bashkit::Result<()> {
        self.0
            .set_permissions(path, mode)
            .await
            .map_err(to_bashkit_error)
    }

    async fn set_modified_time(&self, path: &Path, time: SystemTime) -> bashkit::Result<()> {
        self.0
            .set_modified(path, time)
            .await
            .map_err(to_bashkit_error)
    }
}

fn to_entry(entry: DirectoryEntry) -> bashkit::DirEntry {
    bashkit::DirEntry {
        name: entry.name,
        metadata: to_metadata(entry.metadata),
    }
}

fn to_metadata(metadata: FileMetadata) -> bashkit::Metadata {
    bashkit::Metadata {
        file_type: match metadata.file_type {
            FileType::File => bashkit::FileType::File,
            FileType::Directory => bashkit::FileType::Directory,
            FileType::Symlink => bashkit::FileType::Symlink,
            FileType::Fifo => bashkit::FileType::Fifo,
        },
        size: metadata.size,
        mode: metadata.mode,
        modified: metadata.modified,
        created: metadata.created,
    }
}

fn to_bashkit_error(error: FilesystemError) -> bashkit::Error {
    let kind = match error.kind {
        FilesystemErrorKind::NotFound => ErrorKind::NotFound,
        FilesystemErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
        FilesystemErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
        FilesystemErrorKind::InvalidInput => ErrorKind::InvalidInput,
        FilesystemErrorKind::Unsupported => ErrorKind::Unsupported,
        FilesystemErrorKind::ResourceExhausted => ErrorKind::OutOfMemory,
        FilesystemErrorKind::NotDirectory
        | FilesystemErrorKind::IsDirectory
        | FilesystemErrorKind::DirectoryNotEmpty
        | FilesystemErrorKind::Other => ErrorKind::Other,
    };
    IoError::new(kind, error.message).into()
}
