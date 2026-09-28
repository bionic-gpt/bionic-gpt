use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const HOME_DIR: &str = "/home/user";
pub const WORK_DIR: &str = "/home/user/work";
pub const OUTPUT_DIR: &str = "/home/user/output";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxFile {
    pub path: String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceSnapshot {
    pub key: String,
    pub revision: String,
    pub files: Vec<SandboxFile>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspaceDelta {
    pub upserted: Vec<SandboxFile>,
    pub deleted: Vec<String>,
}

pub(crate) fn mutable_files(files: &[SandboxFile]) -> BTreeMap<String, SandboxFile> {
    files
        .iter()
        .filter(|file| is_mutable(&file.path))
        .map(|file| (file.path.clone(), file.clone()))
        .collect()
}

pub(crate) fn diff_workspace(
    before: BTreeMap<String, SandboxFile>,
    after: &BTreeMap<String, SandboxFile>,
) -> WorkspaceDelta {
    let upserted = after
        .iter()
        .filter(|(path, file)| before.get(*path) != Some(*file))
        .map(|(_, file)| file.clone())
        .collect();
    let after_paths = after.keys().cloned().collect::<BTreeSet<_>>();
    let deleted = before
        .keys()
        .filter(|path| !after_paths.contains(*path))
        .cloned()
        .collect();
    WorkspaceDelta { upserted, deleted }
}

fn is_mutable(path: &str) -> bool {
    path.starts_with(&format!("{WORK_DIR}/")) || path.starts_with(&format!("{OUTPUT_DIR}/"))
}
