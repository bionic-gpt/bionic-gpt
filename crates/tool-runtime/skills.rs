use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableSkill {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSkillFile {
    pub path: String,
    pub contents: Vec<u8>,
}

pub fn format_available_skills_prompt_section(skills: Vec<AvailableSkill>) -> Option<String> {
    if skills.is_empty() {
        return None;
    }

    let mut prompt = String::from(
        "Available skills:\n\
To read a skill, slug the skill name (lowercase, non-alphanumeric to hyphens) and open /home/user/skills/<slug>/SKILL.md.\n",
    );
    for skill in skills {
        prompt.push_str(&format!("- {}: {}\n", skill.name, skill.description));
    }
    prompt.truncate(prompt.trim_end().len());
    Some(prompt)
}

pub fn available_skills_prompt_section_with_custom(
    custom_skills: Vec<db::queries::skills::SkillSummary>,
) -> Option<String> {
    let mut deduplicated_custom_skills: BTreeMap<String, SkillChoice> = BTreeMap::new();
    for skill in custom_skills {
        let slug = slugify_skill_name(&skill.skill_name);
        let choice = SkillChoice {
            skill_id: skill.skill_id,
            is_system: skill.is_system,
            skill: AvailableSkill {
                name: skill.skill_name,
                description: skill.description,
            },
        };
        insert_preferred_skill(&mut deduplicated_custom_skills, slug, choice);
    }

    let skills = deduplicated_custom_skills
        .into_values()
        .map(|choice| choice.skill)
        .collect();
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
            is_system: file.is_system,
            description: file.description,
        })
        .collect();
    available_skills_prompt_section_with_custom(custom_skills)
}

pub fn runtime_skill_files(
    custom_files: Vec<db::queries::skills::SkillFile>,
) -> Vec<RuntimeSkillFile> {
    let selected_skill_ids = preferred_skill_ids_for_files(&custom_files);

    custom_files
        .into_iter()
        .filter(|file| selected_skill_ids.contains(&file.skill_id))
        .map(|file| RuntimeSkillFile {
            path: Path::new(&skill_vfs_directory(
                file.skill_id,
                &file.skill_name,
                file.is_system,
            ))
            .join(file.relative_path)
            .to_string_lossy()
            .to_string(),
            contents: file.object_data,
        })
        .collect()
}

pub fn skill_vfs_directory(_skill_id: i32, name: &str, _is_system: bool) -> String {
    let slug = slugify_skill_name(name);
    format!("/home/user/skills/{slug}")
}

#[derive(Debug, Clone)]
struct SkillChoice {
    skill_id: i32,
    is_system: bool,
    skill: AvailableSkill,
}

fn insert_preferred_skill(
    skills: &mut BTreeMap<String, SkillChoice>,
    slug: String,
    candidate: SkillChoice,
) {
    match skills.get(&slug) {
        Some(existing) if is_preferred_skill(existing, &candidate) => {}
        _ => {
            skills.insert(slug, candidate);
        }
    }
}

fn is_preferred_skill(existing: &SkillChoice, candidate: &SkillChoice) -> bool {
    if existing.is_system != candidate.is_system {
        return existing.is_system;
    }

    existing.skill_id <= candidate.skill_id
}

fn preferred_skill_ids_for_files(files: &[db::queries::skills::SkillFile]) -> BTreeSet<i32> {
    let mut selected: BTreeMap<String, SkillChoice> = BTreeMap::new();
    for file in files {
        let slug = slugify_skill_name(&file.skill_name);
        let choice = SkillChoice {
            skill_id: file.skill_id,
            is_system: file.is_system,
            skill: AvailableSkill {
                name: file.skill_name.clone(),
                description: file.description.clone(),
            },
        };
        insert_preferred_skill(&mut selected, slug, choice);
    }

    selected
        .into_values()
        .map(|choice| choice.skill_id)
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_available_skills_prompt_section() {
        let prompt = format_available_skills_prompt_section(vec![AvailableSkill {
            name: "presentation-builder".to_string(),
            description: "Create slide decks.".to_string(),
        }])
        .unwrap();
        assert!(prompt.contains("Available skills:"));
        assert!(prompt.contains("/home/user/skills/<slug>/SKILL.md"));
        assert!(prompt.contains("- presentation-builder: Create slide decks."));
        assert!(!prompt.contains("<available_skills>"));
        assert!(!prompt.contains("<location>"));
    }

    #[test]
    fn test_skill_vfs_directory_is_stable() {
        assert_eq!(
            skill_vfs_directory(42, "Data Cleanup!", false),
            "/home/user/skills/data-cleanup"
        );
        assert_eq!(
            skill_vfs_directory(42, "Presentation Builder", true),
            "/home/user/skills/presentation-builder"
        );
    }

    #[test]
    fn available_skills_deduplicates_by_slug_preferring_system_skills() {
        let prompt = available_skills_prompt_section_with_custom(vec![
            db::queries::skills::SkillSummary {
                skill_id: 2,
                skill_name: "Data Cleanup".to_string(),
                description: "User skill".to_string(),
                is_system: false,
            },
            db::queries::skills::SkillSummary {
                skill_id: 9,
                skill_name: "Data Cleanup!".to_string(),
                description: "System skill".to_string(),
                is_system: true,
            },
        ])
        .unwrap();

        assert!(prompt.contains("- Data Cleanup!: System skill"));
        assert!(!prompt.contains("User skill"));
    }

    #[test]
    fn available_skills_deduplicates_by_slug_preferring_lowest_id() {
        let prompt = available_skills_prompt_section_with_custom(vec![
            db::queries::skills::SkillSummary {
                skill_id: 7,
                skill_name: "Data Cleanup".to_string(),
                description: "Later skill".to_string(),
                is_system: false,
            },
            db::queries::skills::SkillSummary {
                skill_id: 3,
                skill_name: "Data Cleanup!".to_string(),
                description: "Earlier skill".to_string(),
                is_system: false,
            },
        ])
        .unwrap();

        assert!(prompt.contains("- Data Cleanup!: Earlier skill"));
        assert!(!prompt.contains("Later skill"));
    }
}
