#![allow(non_snake_case)]
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
    // Extract models for components that don't need capabilities
    let models: Vec<ModelWithPrompt> = models_with_capabilities
        .iter()
        .map(|(model, _, _, _)| model.clone())
        .collect();
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
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-model-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Model"
                }
            ),

            super::model_table::ModelTable {
                models: models.clone(),
                team_id: team_id
            }

            // The form to create a model
            super::form::Form {
                team_id: team_id,
                trigger_id: "new-model-form".to_string(),
                name: "".to_string(),
                display_name: "".to_string(),
                model_type: "LLM".to_string(),
                base_url: "".to_string(),
                tpm_limit: 10_000,
                rpm_limit: 10_000,
                api_key: "".to_string(),
                context_size_bytes: 2048,
                description: "".to_string(),
                disclaimer: "AI can make mistakes. Check important information.".to_string(),
                example1: "".to_string(),
                example2: "".to_string(),
                example3: "".to_string(),
                example4: "".to_string(),
                // Default capabilities to false for new models
                has_capability_function_calling: false,
                has_capability_vision: false,
                has_capability_tool_use: false,
            }

            for (model, has_function_calling, has_vision, has_tool_use) in &models_with_capabilities {
                super::form::Form {
                    id: model.id,
                    prompt_id: model.prompt_id,
                    team_id: team_id,
                    display_name: model.display_name.clone(),
                    trigger_id: format!("edit-model-form-{}", model.id),
                    name: model.name.clone(),
                    model_type: super::model_type(model.model_type),
                    base_url: model.base_url.clone(),
                    api_key: model.api_key.clone().unwrap_or("".to_string()),
                    tpm_limit: model.tpm_limit,
                    rpm_limit: model.rpm_limit,
                    context_size_bytes: model.context_size,
                    description: model.description.clone(),
                    disclaimer: model.disclaimer.clone(),
                    example1: model.example1.clone(),
                    example2: model.example2.clone(),
                    example3: model.example3.clone(),
                    example4: model.example4.clone(),
                    // Pass the capabilities
                    has_capability_function_calling: *has_function_calling,
                    has_capability_vision: *has_vision,
                    has_capability_tool_use: *has_tool_use,
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
