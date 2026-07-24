use include_dir::{include_dir, Dir, DirEntry};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableSkill {
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSkillFile {
    pub path: String,
    pub contents: Vec<u8>,
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
    format_available_skills_prompt_section(
        SKILLS
            .iter()
            .map(|skill| AvailableSkill {
                name: skill.name.to_string(),
                description: skill.description.to_string(),
                location: skill.location.to_string(),
            })
            .collect(),
    )
}

pub fn format_available_skills_prompt_section(skills: Vec<AvailableSkill>) -> Option<String> {
    if skills.is_empty() {
        return None;
    }

    let mut prompt = String::from("<available_skills>\n");
    for skill in skills {
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

pub fn available_skills_prompt_section_with_custom(
    custom_skills: Vec<db::queries::skills::SkillSummary>,
) -> Option<String> {
    let mut skills: Vec<AvailableSkill> = SKILLS
        .iter()
        .map(|skill| AvailableSkill {
            name: skill.name.to_string(),
            description: skill.description.to_string(),
            location: skill.location.to_string(),
        })
        .collect();

    let mut deduplicated_custom_skills = BTreeMap::new();
    for skill in custom_skills {
        deduplicated_custom_skills
            .entry(skill.skill_id)
            .or_insert_with(|| {
                let skill_dir = skill_vfs_directory(skill.skill_id, &skill.skill_name);
                AvailableSkill {
                    name: skill.skill_name,
                    description: skill.description,
                    location: format!("{skill_dir}/SKILL.md"),
                }
            });
    }

    skills.extend(deduplicated_custom_skills.into_values());
    format_available_skills_prompt_section(skills)
}

pub fn available_skills_prompt_section_with_custom_files(
    custom_files: Vec<db::queries::skills::SkillFile>,
) -> Option<String> {
    let custom_skills = custom_files
        .into_iter()
        .map(|file| db::queries::skills::SkillSummary {
            skill_id: file.skill_id,
            skill_name: file.skill_name,
            description: file.description,
        })
        .collect();
    available_skills_prompt_section_with_custom(custom_skills)
}

pub fn runtime_skill_files(
    custom_files: Vec<db::queries::skills::SkillFile>,
) -> Vec<RuntimeSkillFile> {
    custom_files
        .into_iter()
        .map(|file| RuntimeSkillFile {
            path: Path::new(&skill_vfs_directory(file.skill_id, &file.skill_name))
                .join(file.relative_path)
                .to_string_lossy()
                .to_string(),
            contents: file.object_data,
        })
        .collect()
}

pub fn skill_vfs_directory(skill_id: i32, name: &str) -> String {
    format!(
        "/home/user/skills/{}-{}",
        skill_id,
        slugify_skill_name(name)
    )
}

fn slugify_skill_name(name: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;

    for ch in name.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            last_was_separator = false;
        } else if !last_was_separator && !slug.is_empty() {
            slug.push('-');
            last_was_separator = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "skill".to_string()
    } else {
        slug
    }
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

    #[test]
    fn test_skill_vfs_directory_is_stable() {
        assert_eq!(
            skill_vfs_directory(42, "Data Cleanup!"),
            "/home/user/skills/42-data-cleanup"
        );
    }
}
