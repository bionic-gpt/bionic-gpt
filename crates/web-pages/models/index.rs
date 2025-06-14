#![allow(non_snake_case)]
use super::model_card::ModelCard;
use crate::app_layout::{Layout, SideBar};
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::models::ModelWithPrompt;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    models_with_capabilities: Vec<(ModelWithPrompt, bool, bool, bool)>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            title: "Models",
            header: rsx!(
                h3 { "Models" }
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
                h1 {
                    class: "text-xl font-semibold mb-4",
                    "Models"
                }
                if models_with_capabilities.is_empty() {
                    div {
                        class: "text-center py-12",
                        p {
                            class: "text-gray-500 mb-4",
                            "You haven't added any models yet."
                        }
                        Button {
                            button_type: ButtonType::Link,
                            href: crate::routes::models::New { team_id }.to_string(),
                            button_scheme: ButtonScheme::Primary,
                            "Create Your First Model"
                        }
                    }
                } else {
                    div {
                        class: "space-y-2",
                        for (model, fc, vis, tool) in &models_with_capabilities {
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

            for (item, _, _, _) in &models_with_capabilities {
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
