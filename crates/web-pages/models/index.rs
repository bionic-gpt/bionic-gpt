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
                    button_type: ButtonType::Link,
                    prefix_image_src: "{button_plus_svg.name}",
                    href: crate::routes::models::New { team_id }.to_string(),
                    button_scheme: ButtonScheme::Primary,
                    "Add Model"
                }
            ),

            super::model_table::ModelTable {
                models: models.clone(),
                team_id: team_id
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
