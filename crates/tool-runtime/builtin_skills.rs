use include_dir::{include_dir, Dir, DirEntry};
use std::path::Path;

static BUILTIN_SKILLS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/builtin_skills");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinSkill {
    pub name: &'static str,
    pub description: &'static str,
    pub location: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinSkillFile {
    pub path: String,
    pub contents: &'static [u8],
}

const SKILLS: &[BuiltinSkill] = &[
    BuiltinSkill {
        name: "dataset-analysis",
        description: "Use assistant datasets for grounded answers with rag-search and rag-read.",
        location: "/home/user/skills/dataset-analysis/SKILL.md",
    },
    BuiltinSkill {
        name: "shell-data-workbench",
        description: "Inspect, filter, summarize, and transform sandbox files with shell tools.",
        location: "/home/user/skills/shell-data-workbench/SKILL.md",
    },
];

pub fn builtin_skills() -> &'static [BuiltinSkill] {
    SKILLS
}

pub fn available_skills_prompt_section() -> Option<String> {
    if SKILLS.is_empty() {
        return None;
    }

    let mut prompt = String::from("<available_skills>\n");
    for skill in SKILLS {
        prompt.push_str("  <skill>\n");
        prompt.push_str(&format!("    <name>{}</name>\n", skill.name));
        prompt.push_str(&format!(
            "    <description>{}</description>\n",
            skill.description
        ));
        prompt.push_str(&format!("    <location>{}</location>\n", skill.location));
        prompt.push_str("  </skill>\n");
    }
    prompt.push_str("</available_skills>");
    Some(prompt)
}

pub fn builtin_skill_files() -> Vec<BuiltinSkillFile> {
    let mut files = Vec::new();
    collect_skill_files(&BUILTIN_SKILLS_DIR, &mut files);
    files
}

fn collect_skill_files(dir: &'static Dir<'static>, files: &mut Vec<BuiltinSkillFile>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(dir) => collect_skill_files(dir, files),
            DirEntry::File(file) => files.push(BuiltinSkillFile {
                path: Path::new("/home/user/skills")
                    .join(file.path())
                    .to_string_lossy()
                    .to_string(),
                contents: file.contents(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_skills_are_registered() {
        let names: Vec<_> = builtin_skills().iter().map(|skill| skill.name).collect();
        assert_eq!(names, vec!["dataset-analysis", "shell-data-workbench"]);
    }

    #[test]
    fn test_builtin_skill_files_are_under_home() {
        let files = builtin_skill_files();
        assert_eq!(files.len(), 2);
        assert!(files
            .iter()
            .all(|file| file.path.starts_with("/home/user/skills/")));
        assert!(files
            .iter()
            .any(|file| file.path.ends_with("/dataset-analysis/SKILL.md")));
    }

    #[test]
    fn test_available_skills_prompt_section() {
        let prompt = available_skills_prompt_section().unwrap();
        assert!(prompt.contains("<available_skills>"));
        assert!(prompt.contains("/home/user/skills/dataset-analysis/SKILL.md"));
    }
}
