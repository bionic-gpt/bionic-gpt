#![allow(non_snake_case)]
use daisy_rsx::*;
use db::IntegrationType;
use dioxus::prelude::*;

#[component]
pub fn Integration(integration_type: IntegrationType) -> Element {
    match integration_type {
        IntegrationType::MCP_Server => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                "MCP Server"
            }
        ),
        IntegrationType::BuiltIn => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                "Built In"
            }
        ),
        IntegrationType::OpenAPI => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                "Open API"
            }
        ),
    }
}
