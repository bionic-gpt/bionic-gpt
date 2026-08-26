#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::card_item::{CardItem, CountLabel};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::skills::Skill;
use db::queries::skills::SkillFile;
use dioxus::prelude::*;
use std::convert::TryFrom;

pub fn page(
    rbac: Rbac,
    team_id: String,
    skills: Vec<Skill>,
    can_set_visibility_to_company: bool,
    locale: &str,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Skills,
            team_id: team_id.clone(),
            rbac,
            title: "Skills",
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Skills".to_string(),
                            href: None
                        }
                    ]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-skill-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Skill"
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: "Skills".to_string(),
                    subtitle: "Upload reusable instructions and helper files for chat sandboxes.".to_string(),
                    is_empty: skills.is_empty(),
                    empty_text: "No skills uploaded yet.".to_string(),
                }

                if !skills.is_empty() {
                    for skill in &skills {
                        SkillCard {
                            skill: skill.clone(),
                            team_id: team_id.clone(),
                            can_set_visibility_to_company,
                        }
                    }
                }

                super::upsert::Upsert {
                    trigger_id: "new-skill-form",
                    id: None,
                    team_id: team_id.clone(),
                    visibility: db::Visibility::Private,
                    can_set_visibility_to_company,
                    requires_file: true,
                }
            }
        }
    };

    crate::render(page)
}

#[component]
fn SkillCard(skill: Skill, team_id: String, can_set_visibility_to_company: bool) -> Element {
    let file_count = usize::try_from(skill.file_count).unwrap_or(0);
    let edit_trigger_id = format!("edit-skill-{}", skill.id);
    let delete_trigger_id = format!("delete-skill-{}", skill.id);
    let avatar_initial = skill.name.chars().next().unwrap_or('S').to_string();

    rsx!(
        div {
            class: "mb-3",
            CardItem {
                class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
                clickable_link: Some(crate::routes::skills::View { team_id: team_id.clone(), id: skill.id }.to_string()),
                avatar_name: Some(avatar_initial),
                title: skill.name.clone(),
                description: Some(rsx!(
                    div {
                        class: "flex flex-wrap items-center gap-2 text-sm text-base-content/70",
                        crate::shared::visibility::VisLabel {
                            visibility: skill.visibility
                        }
                        if skill.is_system {
                            Badge {
                                badge_color: BadgeColor::Info,
                                badge_style: BadgeStyle::Outline,
                                badge_size: BadgeSize::Sm,
                                "System"
                            }
                        }
                        if !skill.description.is_empty() {
                            span { "{skill.description}" }
                        }
                    }
                )),
                count_labels: vec![CountLabel {
                    count: file_count,
                    label: "File".to_string()
                }],
                action: (!skill.is_system).then(|| rsx!(
                        div {
                            class: "flex gap-2",
                            Button {
                                button_scheme: ButtonScheme::Neutral,
                                popover_target: "{edit_trigger_id}",
                                "Edit"
                            }
                            Button {
                                button_scheme: ButtonScheme::Error,
                                popover_target: "{delete_trigger_id}",
                                "Delete"
                            }
                        }
                    )),
            }

            if !skill.is_system {
                super::upsert::Upsert {
                    trigger_id: edit_trigger_id.clone(),
                    id: Some(skill.id),
                    team_id: team_id.clone(),
                    visibility: skill.visibility,
                    can_set_visibility_to_company,
                    requires_file: false,
                }

                ConfirmModal {
                    action: crate::routes::skills::Delete {
                        team_id,
                        id: skill.id,
                    }.to_string(),
                    trigger_id: delete_trigger_id,
                    submit_label: "Delete Skill".to_string(),
                    heading: "Delete skill".to_string(),
                    warning: format!("Delete {}? This removes it from future sandbox runs.", skill.name),
                    hidden_fields: vec![],
                }
            }
        }
    )
}

pub fn detail_page(
    team_id: String,
    rbac: db::authz::Rbac,
    skill: Skill,
    files: Vec<SkillFile>,
    locale: &str,
) -> String {
    let page = rsx! {
        crate::app_layout::Layout {
            section_class: "p-4",
            selected_item: crate::app_layout::SideBar::Skills,
            team_id: team_id.clone(),
            rbac,
            title: skill.name.clone(),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Skills".to_string(),
                            href: Some(crate::routes::skills::Index { team_id: team_id.clone() }.to_string())
                        },
                        BreadcrumbItem { text: skill.name.clone(), href: None }
                    ]
                }
            ),
            div { class: "p-4 max-w-4xl w-full mx-auto",
                h1 { class: "text-2xl font-bold mb-2", "{skill.name}" }
                if !skill.description.is_empty() {
                    p { class: "mb-6 text-base-content/70", "{skill.description}" }
                }
                if files.is_empty() {
                    p { "This skill has no files." }
                } else {
                    div { class: "flex flex-col gap-6",
                        for file in files {
                            SkillFileEditor { team_id: team_id.clone(), file }
                        }
                    }
                }
            }
        }
    };
    crate::render(page)
}

#[component]
fn SkillFileEditor(team_id: String, file: SkillFile) -> Element {
    let text = String::from_utf8(file.object_data.clone()).ok();
    rsx! {
        div { class: "card card-border bg-base-100",
            div { class: "card-body",
                h2 { class: "card-title text-base font-mono", "{file.relative_path}" }
                if let Some(text) = text {
                    form {
                        action: crate::routes::skills::UpdateFile { team_id, id: file.skill_id }.to_string(),
                        method: "post",
                        textarea {
                            class: "textarea textarea-bordered w-full font-mono text-sm",
                            name: "content",
                            rows: "12",
                            "{text}"
                        }
                        input { r#type: "hidden", name: "relative_path", value: "{file.relative_path}" }
                        button { class: "btn btn-primary mt-3", r#type: "submit", "Save" }
                    }
                } else {
                    p { class: "text-sm text-base-content/70", "Binary file preview is not available." }
                }
            }
        }
    }
}
