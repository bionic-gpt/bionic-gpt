#![allow(non_snake_case)]
use super::model_card::ModelCard;
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::models::ModelWithPrompt;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    team_models: Vec<(ModelWithPrompt, bool, bool, bool)>,
    system_models: Vec<(ModelWithPrompt, bool, bool, bool)>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            title: "Models",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Models".into(),
                        href: None
                    }]
                }
                Button {
                    button_type: ButtonType::Link,
                    prefix_image_src: "{button_plus_svg.name}",
                    href: crate::routes::models::New { team_id }.to_string(),
                    button_scheme: ButtonScheme::Primary,
                    "Add Model"
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Models".to_string(),
                    subtitle: "Configure and manage AI models for your assistants and applications.".to_string(),
                    is_empty: team_models.is_empty() && system_models.is_empty(),
                    empty_text: "No models configured yet. Add your first model to start building assistants.".to_string(),
                }

                if !team_models.is_empty() {
                    h2 { class: "font-semibold text-lg mt-6", "Team Models" }
                    div {
                        class: "space-y-2 mt-2",
                        for (model, fc, vis, tool) in &team_models {
                            ModelCard {
                                team_id,
                                model: model.clone(),
                                has_function_calling: *fc,
                                has_vision: *vis,
                                has_tool_use: *tool
                            }
                        }
                    }
                }

                if !system_models.is_empty() {
                    h2 { class: "font-semibold text-lg mt-6", "System Models" }
                    div {
                        class: "space-y-2 mt-2",
                        for (model, fc, vis, tool) in &system_models {
                            ModelCard {
                                team_id,
                                model: model.clone(),
                                has_function_calling: *fc,
                                has_vision: *vis,
                                has_tool_use: *tool
                            }
                        }
                    }
                }
            }

            for (item, _, _, _) in &team_models {
                ConfirmModal {
                    action: crate::routes::models::Delete{team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Model?".to_string(),
                    warning: "Are you sure you want to delete this Model? Deleting a model will also delete any Assistants that use the model".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
                }
            }

            for (item, _, _, _) in &system_models {
                ConfirmModal {
                    action: crate::routes::models::Delete{team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Model?".to_string(),
                    warning: "Are you sure you want to delete this Model? Deleting a model will also delete any Assistants that use the model".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
                }
            }
        }
    };

    crate::render(page)
}
