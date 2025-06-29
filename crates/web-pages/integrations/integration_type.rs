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
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "MCP Server"
            }
        ),
        IntegrationType::BuiltIn => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Built In"
            }
        ),
        IntegrationType::OpenAPI => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Open API"
            }
        ),
    }
}
