#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::projects::ProjectSummary;
use db::Visibility;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    projects: Vec<ProjectSummary>,
    can_set_visibility_to_company: bool,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Projects,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Projects",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Projects".into(),
                        href: None
                    }]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-project",
                    button_scheme: ButtonScheme::Primary,
                    "New Project"
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Projects".to_string(),
                    subtitle: "Group related chats, instructions, and attachments in one place.".to_string(),
                    is_empty: projects.is_empty(),
                    empty_text: "No projects yet. Create one to get started.".to_string(),
                }
                if !projects.is_empty() {
                    div {
                        class: "space-y-2",
                        for project in &projects {
                            super::project_card::ProjectCard {
                                project: project.clone(),
                                team_id,
                            }
                        }
                    }
                }

                for project in &projects {
                    ConfirmModal {
                        action: crate::routes::projects::Delete { team_id, id: project.id }.to_string(),
                        trigger_id: format!("delete-project-{}-{}", project.id, team_id),
                        submit_label: "Delete".to_string(),
                        heading: "Delete this project?".to_string(),
                        warning: "Are you sure you want to delete this project?".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), team_id.to_string()),
                            ("id".into(), project.id.to_string()),
                        ],
                    }
                    super::upsert::Upsert {
                        id: Some(project.id),
                        trigger_id: format!("edit-project-{}-{}", project.id, team_id),
                        name: project.name.clone(),
                        instructions: project.instructions.clone(),
                        visibility: project.visibility,
                        can_set_visibility_to_company,
                        team_id,
                    }
                }

                super::upsert::Upsert {
                    id: None,
                    trigger_id: "new-project",
                    name: "".to_string(),
                    instructions: "".to_string(),
                    visibility: Visibility::Private,
                    can_set_visibility_to_company,
                    team_id,
                }
            }
        }
    };

    crate::render(page)
}
