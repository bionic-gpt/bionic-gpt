#![allow(non_snake_case)]
use daisy_rsx::*;
use db::{ApiKeyConnection, Integration, Oauth2Connection};
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationForm {
    pub prompt_id: i32,
    pub prompt_name: String,
    pub selected_integration_ids: Vec<i32>,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub integrations: Vec<IntegrationWithAuthInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationWithAuthInfo {
    pub integration: Integration,
    pub requires_api_key: bool,
    pub requires_oauth2: bool,
    pub has_connections: bool,
    pub api_key_connections: Vec<ApiKeyConnection>,
    pub oauth2_connections: Vec<Oauth2Connection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationStatus {
    Available,
    RequiresAPIKey,
    RequiresOauth2Key,
    Active,
}

#[component]
pub fn Status(status: IntegrationStatus) -> Element {
    match status {
        IntegrationStatus::Active => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Active"
            }
        ),
        IntegrationStatus::RequiresAPIKey => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Warning,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Missing API Key"
            }
        ),
        IntegrationStatus::RequiresOauth2Key => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Warning,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Missing Oauth2"
            }
        ),
        IntegrationStatus::Available => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Available"
            }
        ),
    }
}

pub fn determine_status(info: &IntegrationWithAuthInfo, connected: bool) -> IntegrationStatus {
    if connected {
        IntegrationStatus::Active
    } else if info.requires_api_key && !info.has_connections {
        IntegrationStatus::RequiresAPIKey
    } else if info.requires_oauth2 && !info.has_connections {
        IntegrationStatus::RequiresOauth2Key
    } else {
        IntegrationStatus::Available
    }
}
