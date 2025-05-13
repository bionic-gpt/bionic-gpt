#![allow(non_snake_case)]
use daisy_rsx::*;
use db::IntegrationType;
use dioxus::prelude::*;

#[component]
pub fn Integration(integration_type: IntegrationType) -> Element {
    match integration_type {
        IntegrationType::MCP_Server => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "MCP Server"
            }
        ),
        IntegrationType::BuiltIn => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "Built In"
            }
        ),
        IntegrationType::OpenAPI => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "Open API"
            }
        ),
    }
}
