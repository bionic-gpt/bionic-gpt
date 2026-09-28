use crate::{
    error, OpenApiSpec, SandboxError, SandboxFile, SandboxToolError, ToolFiles, HOME_DIR,
    OUTPUT_DIR, WORK_DIR,
};
use async_trait::async_trait;
use bashkit::{FileSystem, FileType};
use std::collections::BTreeMap;
use std::path::{Component, Path};
use std::sync::Arc;

pub(super) struct FsToolFiles(pub(super) Arc<dyn FileSystem>);

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

pub(super) async fn seed_files<'a>(
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

pub(super) async fn seed_openapi_specs(
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

pub(super) async fn write_checked(
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

pub(super) fn checked_path(path: &str) -> Result<&Path, SandboxError> {
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

pub(super) async fn collect_mutable_files(
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
            if entry.metadata.file_type == FileType::Directory {
                pending.push(path);
            } else if entry.metadata.file_type == FileType::File {
                let path = path.to_string_lossy().to_string();
                let contents = fs.read_file(Path::new(&path)).await.map_err(error)?;
                files.insert(path.clone(), SandboxFile { path, contents });
            }
        }
    }
    Ok(files)
}
