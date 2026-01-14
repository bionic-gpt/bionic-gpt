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
    setup_required: bool,
    models_with_capabilities: Vec<(ModelWithPrompt, bool, bool, bool, bool)>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            setup_required: setup_required,
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
                if setup_required {
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-6 flex flex-col gap-2",
                        div { class: "font-semibold", "Model setup required" }
                        div { class: "text-sm opacity-90", "Add at least one LLM model and one Embeddings model to continue." }
                    }
                }
                SectionIntroduction {
                    header: "Models".to_string(),
                    subtitle: "Configure and manage AI models for your assistants and applications.".to_string(),
                    is_empty: models_with_capabilities.is_empty(),
                    empty_text: "No models configured yet. Add your first model to start building assistants.".to_string(),
                }
                if !models_with_capabilities.is_empty() {
                    div {
                        class: "space-y-2",
                        for (model, fc, vis, tool, guard) in &models_with_capabilities {
                            ModelCard {
                                team_id,
                                model: model.clone(),
                                has_function_calling: *fc,
                                has_vision: *vis,
                                has_tool_use: *tool,
                                has_guard: *guard
                            }
                        }
                    }
                }
            }

            for (item, _, _, _, _) in &models_with_capabilities {
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
