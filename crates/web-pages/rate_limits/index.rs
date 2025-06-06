#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, Model, RateLimit};
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32, rate_limits: Vec<RateLimit>, models: Vec<Model>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::RateLimits,
            team_id: team_id,
            rbac: rbac,
            title: "Rate Limits",
            header: rsx! {
                h3 { "Rate Limits" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-limit",
                    button_scheme: ButtonScheme::Primary,
                    "Add Limit"
                }
            },
            BlankSlate {
                heading: "Bionic can assign token limits based on a users role.",
                visual: limits_svg.name,
                description: "Roles are assigned in your identity system and mapped here to limits"
            }

            super::RateTable { rate_limits: rate_limits.clone(), team_id }

            for item in rate_limits {
                ConfirmModal {
                    action: crate::routes::rate_limits::Delete {team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Rate Limit?".to_string(),
                    warning: "Are you sure you want to delete this rate limit?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
                }
            }

            // Our pop out drawer to add limits
            super::form::Form {
                team_id: team_id,
                models
            }
        }
    };

    crate::render(page)
}
