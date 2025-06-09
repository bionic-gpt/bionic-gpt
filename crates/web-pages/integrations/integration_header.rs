#![allow(non_snake_case)]
use crate::routes;
use crate::ConfirmModal;
use assets::files::{button_edit_svg, menu_delete_svg};
use daisy_rsx::*;
use db::{authz::Rbac, Integration};
use dioxus::prelude::*;

#[component]
pub fn IntegrationHeader(
    team_id: i32,
    rbac: Rbac,
    integration: Integration,
    logo_url: String,
    description: Option<String>,
) -> Element {
    let popover_target = format!("delete-integration-{}", integration.id);

    rsx! {
        div {
            class: "flex justify-between",
            div {
                class: "flex items-center",
                img {
                    class: "w-12 h-12 object-contain border border-neutral-content rounded p-2",
                    src: "{logo_url}",
                    width: "48",
                    height: "48"
                }
                div {
                    class: "ml-4",
                    h2 {
                        class: "text-xl font-semibold",
                        "{integration.name.clone()}"
                    }
                    if let Some(description) = description {
                        p {
                            class: "text-sm text-gray-700 whitespace-pre-wrap break-words mt-1",
                            "{description}"
                        }
                    }
                }
            }
            div {
                class: "flex flex-col justify-center",
                div {
                    class: "flex gap-4",
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_edit_svg.name}",
                        href: routes::integrations::Edit{team_id, id: integration.id}.to_string(),
                        button_style: ButtonStyle::Outline,
                        "Edit"
                    }
                    Button {
                        prefix_image_src: "{menu_delete_svg.name}",
                        popover_target: popover_target.clone(),
                        button_scheme: ButtonScheme::Error
                    }
                    ConfirmModal {
                        action: crate::routes::integrations::Delete{team_id, id: integration.id}.to_string(),
                        trigger_id: popover_target,
                        submit_label: "Delete".to_string(),
                        heading: "Delete this Integration?".to_string(),
                        warning: "Are you sure you want to delete this Integration?".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), team_id.to_string()),
                            ("id".into(), integration.id.to_string()),
                        ],
                    }
                }
            }
        }
    }
}
