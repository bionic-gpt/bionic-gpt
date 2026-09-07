//! Skills shipped with the Bionic runtime.

use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillFile {
    pub path: &'static str,
    pub contents: &'static [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub files: &'static [SkillFile],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

pub fn parse_skill_frontmatter(bytes: &[u8]) -> Result<SkillMetadata, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| "SKILL.md must be valid UTF-8 to read its metadata".to_string())?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let normalized = text.replace("\r\n", "\n");
    let rest = normalized
        .strip_prefix("---\n")
        .ok_or_else(|| "SKILL.md must start with frontmatter".to_string())?;
    let (frontmatter, _body) = rest
        .split_once("\n---\n")
        .ok_or_else(|| "SKILL.md frontmatter is not closed".to_string())?;

    let mut name = None;
    let mut description = None;
    for line in frontmatter.lines() {
        let (key, value) = line
            .split_once(':')
            .ok_or_else(|| "SKILL.md frontmatter contains an invalid line".to_string())?;
        let value = value.trim().trim_matches(['"', '\'']).trim().to_string();
        match key.trim() {
            "name" => name = Some(value),
            "description" => description = Some(value),
            _ => {}
        }
    }

    let required = |field: &'static str, value: Option<String>| {
        value
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("SKILL.md frontmatter field `{field}` is required"))
    };
    Ok(SkillMetadata {
        name: required("name", name)?,
        description: required("description", description)?,
    })
}

static DATASET_ANALYSIS_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/dataset-analysis/SKILL.md"),
}];

static SHELL_DATA_WORKBENCH_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/shell-data-workbench/SKILL.md"),
}];

static STRUCTURED_EXTRACTION_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/structured-extraction/SKILL.md"),
}];

static DOCUMENT_COMPARISON_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/document-comparison/SKILL.md"),
}];

static DOCUMENT_COAUTHORING_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/document-coauthoring/SKILL.md"),
}];

static IMAGE_ANALYSIS_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/image-analysis/SKILL.md"),
}];

static DATABASE_FILES: &[SkillFile] = &[SkillFile {
    path: "SKILL.md",
    contents: include_bytes!("skills/database/SKILL.md"),
}];

static PRESENTATION_BUILDER_FILES: &[SkillFile] = &[
    SkillFile {
        path: "SKILL.md",
        contents: include_bytes!("skills/presentation-builder/SKILL.md"),
    },
    SkillFile {
        path: "bin/build-reveal-canvas.sh",
        contents: include_bytes!("skills/presentation-builder/bin/build-reveal-canvas.sh"),
    },
    SkillFile {
        path: "reveal/reveal.js",
        contents: include_bytes!("skills/presentation-builder/reveal/reveal.js"),
    },
    SkillFile {
        path: "reveal/reveal.css",
        contents: include_bytes!("skills/presentation-builder/reveal/reveal.css"),
    },
    SkillFile {
        path: "reveal/reset.css",
        contents: include_bytes!("skills/presentation-builder/reveal/reset.css"),
    },
    SkillFile {
        path: "reveal/theme/serif.css",
        contents: include_bytes!("skills/presentation-builder/reveal/theme/serif.css"),
    },
    SkillFile {
        path: "LICENSE.reveal.js",
        contents: include_bytes!("skills/presentation-builder/LICENSE.reveal.js"),
    },
];

static SKILL_FILE_SETS: &[&[SkillFile]] = &[
    DATABASE_FILES,
    DATASET_ANALYSIS_FILES,
    DOCUMENT_COAUTHORING_FILES,
    DOCUMENT_COMPARISON_FILES,
    IMAGE_ANALYSIS_FILES,
    PRESENTATION_BUILDER_FILES,
    SHELL_DATA_WORKBENCH_FILES,
    STRUCTURED_EXTRACTION_FILES,
];

static SKILLS: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    SKILL_FILE_SETS
        .iter()
        .map(|files| {
            let skill_file = files
                .iter()
                .find(|file| file.path == "SKILL.md")
                .expect("built-in skill must contain SKILL.md");
            let metadata = parse_skill_frontmatter(skill_file.contents)
                .expect("built-in SKILL.md must contain valid metadata");
            Skill {
                name: metadata.name,
                description: metadata.description,
                files,
            }
        })
        .collect()
});

pub fn all() -> &'static [Skill] {
    SKILLS.as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_all_builtin_skills_and_skill_files() {
        assert_eq!(all().len(), 8);
        assert!(all().iter().all(|skill| {
            skill.files.iter().any(|file| file.path == "SKILL.md")
                && skill.files.iter().all(|file| !file.contents.is_empty())
        }));
    }

    #[test]
    fn presentation_skill_contains_runtime_assets() {
        let skill = all()
            .iter()
            .find(|skill| skill.name == "presentation-builder")
            .unwrap();
        assert!(skill
            .files
            .iter()
            .any(|file| file.path == "reveal/reveal.js"));
        assert!(skill
            .files
            .iter()
            .any(|file| file.path == "bin/build-reveal-canvas.sh"));
    }

    #[test]
    fn requires_complete_frontmatter() {
        assert!(parse_skill_frontmatter(b"# Missing metadata").is_err());
        assert!(parse_skill_frontmatter(b"---\nname: test\n---\n# Test").is_err());
        assert!(parse_skill_frontmatter(b"---\nname: test\ndescription: \n---\n# Test").is_err());
    }
}
