use std::collections::BTreeMap;
use std::path::Path;

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
    let mut skills = Vec::new();
    let mut deduplicated_custom_skills = BTreeMap::new();
    for skill in custom_skills {
        deduplicated_custom_skills
            .entry(skill.skill_id)
            .or_insert_with(|| {
                let skill_dir =
                    skill_vfs_directory(skill.skill_id, &skill.skill_name, skill.is_system);
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
            is_system: file.is_system,
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

pub fn skill_vfs_directory(skill_id: i32, name: &str, is_system: bool) -> String {
    let slug = slugify_skill_name(name);
    if is_system {
        format!("/home/user/skills/{slug}")
    } else {
        format!("/home/user/skills/{skill_id}-{slug}")
    }
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
            location: "/home/user/skills/presentation-builder/SKILL.md".to_string(),
        }])
        .unwrap();
        assert!(prompt.contains("<available_skills>"));
        assert!(prompt.contains("/home/user/skills/presentation-builder/SKILL.md"));
    }

    #[test]
    fn test_skill_vfs_directory_is_stable() {
        assert_eq!(
            skill_vfs_directory(42, "Data Cleanup!", false),
            "/home/user/skills/42-data-cleanup"
        );
        assert_eq!(
            skill_vfs_directory(42, "Presentation Builder", true),
            "/home/user/skills/presentation-builder"
        );
    }
}
