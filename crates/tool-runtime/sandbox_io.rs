use async_trait::async_trait;
use bashkit::FileSystem;
use sandbox::{
    DirectoryEntry, FileMetadata, FileType, FilesystemError, FilesystemErrorKind,
    SandboxFilesystem, WriteMode,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

/// Transitional adapter for the application-owned Bashkit VFS. It lets the
/// execution crate consume only the provider-neutral filesystem contract while
/// storage routes are moved out of the eager in-memory builder.
pub(crate) struct RuntimeFilesystem {
    inner: Arc<dyn FileSystem>,
}

impl RuntimeFilesystem {
    pub(crate) fn new(inner: Arc<dyn FileSystem>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl SandboxFilesystem for RuntimeFilesystem {
    async fn stat(&self, path: &Path) -> Result<FileMetadata, FilesystemError> {
        self.inner
            .stat(path)
            .await
            .map(to_metadata)
            .map_err(fs_error)
    }

    async fn read_dir(&self, path: &Path) -> Result<Vec<DirectoryEntry>, FilesystemError> {
        self.inner
            .read_dir(path)
            .await
            .map(|entries| {
                entries
                    .into_iter()
                    .map(|entry| DirectoryEntry {
                        name: entry.name,
                        metadata: to_metadata(entry.metadata),
                    })
                    .collect()
            })
            .map_err(fs_error)
    }

    async fn read(&self, path: &Path) -> Result<Vec<u8>, FilesystemError> {
        self.inner.read_file(path).await.map_err(fs_error)
    }

    async fn write(
        &self,
        path: &Path,
        contents: &[u8],
        mode: WriteMode,
    ) -> Result<(), FilesystemError> {
        match mode {
            WriteMode::Truncate => self.inner.write_file(path, contents).await,
            WriteMode::Append => self.inner.append_file(path, contents).await,
        }
        .map_err(fs_error)
    }

    async fn create_dir(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError> {
        self.inner.mkdir(path, recursive).await.map_err(fs_error)
    }

    async fn remove(&self, path: &Path, recursive: bool) -> Result<(), FilesystemError> {
        self.inner.remove(path, recursive).await.map_err(fs_error)
    }

    async fn rename(&self, from: &Path, to: &Path) -> Result<(), FilesystemError> {
        self.inner.rename(from, to).await.map_err(fs_error)
    }

    async fn copy(&self, from: &Path, to: &Path) -> Result<(), FilesystemError> {
        self.inner.copy(from, to).await.map_err(fs_error)
    }

    async fn symlink(&self, target: &Path, link: &Path) -> Result<(), FilesystemError> {
        self.inner.symlink(target, link).await.map_err(fs_error)
    }

    async fn read_link(&self, path: &Path) -> Result<PathBuf, FilesystemError> {
        self.inner.read_link(path).await.map_err(fs_error)
    }

    async fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), FilesystemError> {
        self.inner.chmod(path, mode).await.map_err(fs_error)
    }

    async fn set_modified(&self, path: &Path, modified: SystemTime) -> Result<(), FilesystemError> {
        self.inner
            .set_modified_time(path, modified)
            .await
            .map_err(fs_error)
    }

    async fn exists(&self, path: &Path) -> Result<bool, FilesystemError> {
        self.inner.exists(path).await.map_err(fs_error)
    }
}

fn to_metadata(metadata: bashkit::Metadata) -> FileMetadata {
    FileMetadata {
        file_type: match metadata.file_type {
            bashkit::FileType::File => FileType::File,
            bashkit::FileType::Directory => FileType::Directory,
            bashkit::FileType::Symlink => FileType::Symlink,
            bashkit::FileType::Fifo => FileType::Fifo,
        },
        size: metadata.size,
        mode: metadata.mode,
        modified: metadata.modified,
        created: metadata.created,
    }
}

fn fs_error(error: bashkit::Error) -> FilesystemError {
    FilesystemError::new(FilesystemErrorKind::Other, error.to_string())
}
