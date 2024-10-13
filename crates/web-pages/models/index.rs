#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::models::Model;
use dioxus::prelude::*;

#[component]
pub fn Page(team_id: i32, rbac: Rbac, models: Vec<Model>) -> Element {
    rsx! {
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
                    drawer_trigger: "new-model-form",
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
                model_type: "LLM".to_string(),
                base_url: "".to_string(),
                tpm_limit: 10_000,
                rpm_limit: 10_000,
                api_key: "".to_string(),
                context_size_bytes: 2048,
                disclaimer: "LLMs can make mistakes. Check important info.".to_string(),
                example1: None,
                example2: None,
                example3: None,
                example4: None,
            }

            for model in &models {
                super::form::Form {
                    id: model.id,
                    team_id: team_id,
                    trigger_id: format!("edit-model-form-{}", model.id),
                    name: model.name.clone(),
                    model_type: super::model_type(model.model_type),
                    base_url: model.base_url.clone(),
                    api_key: model.api_key.clone().unwrap_or("".to_string()),
                    tpm_limit: model.tpm_limit,
                    rpm_limit: model.rpm_limit,
                    context_size_bytes: model.context_size,
                    disclaimer: "LLMs can make mistakes. Check important info.".to_string(),
                    example1: None,
                    example2: None,
                    example3: None,
                    example4: None,
                }
            }

            for item in &models {
                super::delete::DeleteDrawer {
                    team_id: team_id,
                    id: item.id,
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                }
            }
        }
    }
}
