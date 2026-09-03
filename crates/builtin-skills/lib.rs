//! Skills shipped with the Bionic runtime.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillFile {
    pub path: &'static str,
    pub contents: &'static [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skill {
    pub name: &'static str,
    pub description: &'static str,
    pub files: &'static [SkillFile],
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

static SKILLS: &[Skill] = &[
    Skill {
        name: "database",
        description: "Create, query, update, or maintain SQLite databases and structured persistent data.",
        files: DATABASE_FILES,
    },
    Skill {
        name: "dataset-analysis",
        description: "Use assistant datasets for grounded answers with rag-search and rag-read.",
        files: DATASET_ANALYSIS_FILES,
    },
    Skill {
        name: "document-coauthoring",
        description: "Guide users through structured co-authoring of proposals, specifications, RFCs, and decision documents.",
        files: DOCUMENT_COAUTHORING_FILES,
    },
    Skill {
        name: "document-comparison",
        description: "Compare extracted documents against rubrics and reference documents with traceable evidence.",
        files: DOCUMENT_COMPARISON_FILES,
    },
    Skill {
        name: "image-analysis",
        description: "Use image evidence to produce domain-aware answers that distinguish observations and uncertainty.",
        files: IMAGE_ANALYSIS_FILES,
    },
    Skill {
        name: "presentation-builder",
        description: "Create reveal.js slide decks and presentation-style visual artifacts as generated HTML canvas files.",
        files: PRESENTATION_BUILDER_FILES,
    },
    Skill {
        name: "shell-data-workbench",
        description: "Inspect, filter, summarize, and transform sandbox files with shell tools.",
        files: SHELL_DATA_WORKBENCH_FILES,
    },
    Skill {
        name: "structured-extraction",
        description: "Extract structured, source-located evidence from uploaded documents using runtime document APIs.",
        files: STRUCTURED_EXTRACTION_FILES,
    },
];

pub fn all() -> &'static [Skill] {
    SKILLS
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
}
