#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::card_item::{CardItem, CountLabel};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::skills::Skill;
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
                class: Some("w-full".into()),
                avatar_name: Some(avatar_initial),
                title: skill.name.clone(),
                description: Some(rsx!(
                    div {
                        class: "flex flex-wrap items-center gap-2 text-sm text-base-content/70",
                        crate::assistants::visibility::VisLabel {
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
