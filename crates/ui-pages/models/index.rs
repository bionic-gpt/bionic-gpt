#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::queries::models::Model;
use db::{ModelType, TopUser};
use dioxus::prelude::*;

#[inline_props]
pub fn Page(
    cx: Scope<Props>,
    team_id: i32,
    is_sys_admin: bool,
    models: Vec<Model>,
    top_users: Vec<TopUser>,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Models,
            team_id: *team_id,
            is_sys_admin: *is_sys_admin,
            title: "Models",
            header: cx.render(rsx!(
                h3 { "Models" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-model-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Model"
                }
            )),

            super::model_table::ModelTable {
                models: &models,
                team_id: *team_id
            }

            super::top_users_table::TopUserTable {
                top_users: &top_users
            }

            // The form to create a model
            super::form::Form {
                team_id: *team_id,
                trigger_id: "new-model-form".to_string(),
                name: "".to_string(),
                model_type: "LLM".to_string(),
                base_url: "".to_string(),
                billion_parameters: 7,
                api_key: "".to_string(),
                context_size_bytes: 2048,
            }


            models.iter().map(|model| {
                // The form to edit a model
                let model_type = if model.model_type == ModelType::LLM {
                    "LLM"
                } else {
                    "Embeddings"
                };
                cx.render(rsx!(
                    super::form::Form {
                        id: model.id,
                        team_id: *team_id,
                        trigger_id: format!("edit-model-form-{}", model.id),
                        name: model.name.clone(),
                        model_type: model_type.to_string(),
                        base_url: model.base_url.clone(),
                        api_key: model.api_key.clone().unwrap_or("".to_string()),
                        billion_parameters: model.billion_parameters,
                        context_size_bytes: model.context_size,
                    }
                ))
            })
        }
    })
}
pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
