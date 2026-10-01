use async_trait::async_trait;
use bashkit::{FileSystem, FileSystemExt};
use std::collections::{BTreeSet, HashMap};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

type LoadFuture = Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + Send>>;
type Loader = Arc<dyn Fn() -> LoadFuture + Send + Sync>;

#[derive(Clone)]
struct LazyEntry {
    size: u64,
    loader: Loader,
}

/// Async lazy-file layer over Bashkit's mutable in-memory filesystem.
/// Directory metadata is available immediately; object bytes are fetched only
/// when a command actually reads the file.
pub(crate) struct LazyFilesystem {
    inner: Arc<dyn FileSystem>,
    lazy: tokio::sync::Mutex<HashMap<PathBuf, LazyEntry>>,
    changed: tokio::sync::Mutex<BTreeSet<PathBuf>>,
    protected: tokio::sync::RwLock<Vec<PathBuf>>,
}

impl LazyFilesystem {
    pub(crate) fn new(inner: Arc<dyn FileSystem>) -> Self {
        Self {
            inner,
            lazy: tokio::sync::Mutex::new(HashMap::new()),
            changed: tokio::sync::Mutex::new(BTreeSet::new()),
            protected: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    pub(crate) async fn changed_files(&self) -> Vec<PathBuf> {
        self.changed.lock().await.iter().cloned().collect()
    }

    pub(crate) async fn protect(&self, roots: impl IntoIterator<Item = PathBuf>) {
        self.protected.write().await.extend(roots);
    }

    async fn require_writable(&self, path: &Path) -> bashkit::Result<()> {
        if self
            .protected
            .read()
            .await
            .iter()
            .any(|root| path.starts_with(root))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("{} is read-only", path.display()),
            )
            .into());
        }
        Ok(())
    }

    pub(crate) async fn register<F, Fut>(
        &self,
        path: &Path,
        size: u64,
        loader: F,
    ) -> bashkit::Result<()>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<u8>, String>> + Send + 'static,
    {
        if let Some(parent) = path.parent() {
            self.inner.mkdir(parent, true).await?;
        }
        self.inner.write_file(path, &[]).await?;
        self.lazy.lock().await.insert(
            path.to_path_buf(),
            LazyEntry {
                size,
                loader: Arc::new(move || Box::pin(loader())),
            },
        );
        Ok(())
    }

    async fn materialize(&self, path: &Path) -> bashkit::Result<()> {
        let mut lazy = self.lazy.lock().await;
        let Some(entry) = lazy.get(path).cloned() else {
            return Ok(());
        };
        let bytes = (entry.loader)().await.map_err(std::io::Error::other)?;
        self.inner.write_file(path, &bytes).await?;
        lazy.remove(path);
        Ok(())
    }

    async fn descendant_files(&self, root: &Path) -> bashkit::Result<Vec<PathBuf>> {
        if self.inner.stat(root).await?.file_type != bashkit::FileType::Directory {
            return Ok(vec![root.to_path_buf()]);
        }
        let mut pending = vec![root.to_path_buf()];
        let mut files = Vec::new();
        while let Some(directory) = pending.pop() {
            for entry in self.inner.read_dir(&directory).await? {
                let path = directory.join(entry.name);
                match entry.metadata.file_type {
                    bashkit::FileType::Directory => pending.push(path),
                    bashkit::FileType::File => files.push(path),
                    _ => {}
                }
            }
        }
        Ok(files)
    }
}

impl FileSystemExt for LazyFilesystem {}

#[async_trait]
impl FileSystem for LazyFilesystem {
    async fn read_file(&self, path: &Path) -> bashkit::Result<Vec<u8>> {
        self.materialize(path).await?;
        self.inner.read_file(path).await
    }

    async fn write_file(&self, path: &Path, content: &[u8]) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.lazy.lock().await.remove(path);
        self.inner.write_file(path, content).await?;
        self.changed.lock().await.insert(path.to_path_buf());
        Ok(())
    }

    async fn append_file(&self, path: &Path, content: &[u8]) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.materialize(path).await?;
        self.inner.append_file(path, content).await?;
        self.changed.lock().await.insert(path.to_path_buf());
        Ok(())
    }

    async fn mkdir(&self, path: &Path, recursive: bool) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.inner.mkdir(path, recursive).await
    }

    async fn remove(&self, path: &Path, recursive: bool) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.lazy.lock().await.retain(|candidate, _| {
            !(candidate == path || (recursive && candidate.starts_with(path)))
        });
        self.inner.remove(path, recursive).await?;
        self.changed.lock().await.insert(path.to_path_buf());
        Ok(())
    }

    async fn stat(&self, path: &Path) -> bashkit::Result<bashkit::Metadata> {
        let mut metadata = self.inner.stat(path).await?;
        if let Some(entry) = self.lazy.lock().await.get(path) {
            metadata.size = entry.size;
        }
        Ok(metadata)
    }

    async fn read_dir(&self, path: &Path) -> bashkit::Result<Vec<bashkit::DirEntry>> {
        let mut entries = self.inner.read_dir(path).await?;
        let lazy = self.lazy.lock().await;
        for entry in &mut entries {
            if let Some(source) = lazy.get(&path.join(&entry.name)) {
                entry.metadata.size = source.size;
            }
        }
        Ok(entries)
    }

    async fn exists(&self, path: &Path) -> bashkit::Result<bool> {
        self.inner.exists(path).await
    }

    async fn rename(&self, from: &Path, to: &Path) -> bashkit::Result<()> {
        self.require_writable(from).await?;
        self.require_writable(to).await?;
        let moved_files = self.descendant_files(from).await?;
        self.inner.rename(from, to).await?;
        let mut lazy = self.lazy.lock().await;
        let moves = lazy
            .iter()
            .filter_map(|(path, entry)| {
                path.strip_prefix(from)
                    .ok()
                    .map(|suffix| (path.clone(), to.join(suffix), entry.clone()))
            })
            .collect::<Vec<_>>();
        for (old, new, entry) in moves {
            lazy.remove(&old);
            lazy.insert(new, entry);
        }
        let mut changed = self.changed.lock().await;
        changed.insert(from.to_path_buf());
        for old in moved_files {
            let suffix = old.strip_prefix(from).unwrap_or(Path::new(""));
            changed.insert(to.join(suffix));
        }
        Ok(())
    }

    async fn copy(&self, from: &Path, to: &Path) -> bashkit::Result<()> {
        self.require_writable(to).await?;
        self.materialize(from).await?;
        self.inner.copy(from, to).await?;
        self.changed.lock().await.insert(to.to_path_buf());
        Ok(())
    }

    async fn symlink(&self, target: &Path, link: &Path) -> bashkit::Result<()> {
        self.require_writable(link).await?;
        self.inner.symlink(target, link).await
    }
    async fn read_link(&self, path: &Path) -> bashkit::Result<PathBuf> {
        self.inner.read_link(path).await
    }
    async fn chmod(&self, path: &Path, mode: u32) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.inner.chmod(path, mode).await
    }
    async fn set_modified_time(&self, path: &Path, time: SystemTime) -> bashkit::Result<()> {
        self.require_writable(path).await?;
        self.inner.set_modified_time(path, time).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bashkit::InMemoryFs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn listing_and_stat_do_not_load_content() {
        let fs = LazyFilesystem::new(Arc::new(InMemoryFs::new()));
        let loads = Arc::new(AtomicUsize::new(0));
        let counter = loads.clone();
        fs.register(
            Path::new("/home/user/attachments/report.pdf"),
            123,
            move || {
                let counter = counter.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(b"contents".to_vec())
                }
            },
        )
        .await
        .unwrap();

        assert_eq!(
            fs.stat(Path::new("/home/user/attachments/report.pdf"))
                .await
                .unwrap()
                .size,
            123
        );
        assert_eq!(
            fs.read_dir(Path::new("/home/user/attachments"))
                .await
                .unwrap()[0]
                .metadata
                .size,
            123
        );
        assert_eq!(loads.load(Ordering::SeqCst), 0);
        assert_eq!(
            fs.read_file(Path::new("/home/user/attachments/report.pdf"))
                .await
                .unwrap(),
            b"contents"
        );
        assert_eq!(loads.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn overwriting_a_lazy_file_never_loads_its_base() {
        let fs = LazyFilesystem::new(Arc::new(InMemoryFs::new()));
        let loads = Arc::new(AtomicUsize::new(0));
        let counter = loads.clone();
        fs.register(Path::new("/home/user/work/note.txt"), 10, move || {
            let counter = counter.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(b"old".to_vec())
            }
        })
        .await
        .unwrap();
        fs.write_file(Path::new("/home/user/work/note.txt"), b"new")
            .await
            .unwrap();
        assert_eq!(
            fs.read_file(Path::new("/home/user/work/note.txt"))
                .await
                .unwrap(),
            b"new"
        );
        assert_eq!(loads.load(Ordering::SeqCst), 0);
        assert_eq!(
            fs.changed_files().await,
            vec![PathBuf::from("/home/user/work/note.txt")]
        );
    }

    #[tokio::test]
    async fn renaming_a_directory_journals_each_destination_file() {
        let fs = LazyFilesystem::new(Arc::new(InMemoryFs::new()));
        fs.mkdir(Path::new("/home/user/work/source/nested"), true)
            .await
            .unwrap();
        fs.write_file(Path::new("/home/user/work/source/a.txt"), b"a")
            .await
            .unwrap();
        fs.write_file(Path::new("/home/user/work/source/nested/b.txt"), b"b")
            .await
            .unwrap();
        fs.changed.lock().await.clear();

        fs.rename(
            Path::new("/home/user/work/source"),
            Path::new("/home/user/work/target"),
        )
        .await
        .unwrap();

        assert_eq!(
            fs.changed_files().await,
            vec![
                PathBuf::from("/home/user/work/source"),
                PathBuf::from("/home/user/work/target/a.txt"),
                PathBuf::from("/home/user/work/target/nested/b.txt"),
            ]
        );
    }

    #[tokio::test]
    async fn protected_mounts_are_read_only() {
        let fs = LazyFilesystem::new(Arc::new(InMemoryFs::new()));
        fs.mkdir(Path::new("/home/user/skills/example"), true)
            .await
            .unwrap();
        fs.write_file(Path::new("/home/user/skills/example/SKILL.md"), b"docs")
            .await
            .unwrap();
        fs.protect([PathBuf::from("/home/user/skills")]).await;

        assert!(fs
            .write_file(Path::new("/home/user/skills/example/SKILL.md"), b"changed")
            .await
            .is_err());
        assert!(fs
            .remove(Path::new("/home/user/skills/example"), true)
            .await
            .is_err());
        fs.mkdir(Path::new("/home/user/work"), true).await.unwrap();
        fs.write_file(Path::new("/home/user/work/note.txt"), b"allowed")
            .await
            .unwrap();
    }
}
