#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Guardrails,
            team_id: team_id,
            rbac: rbac,
            title: "Guardrails",
            header: rsx! {
                h3 { "Guardrails" }
            },
            BlankSlate {
                heading: "GuardRails protect your bionic installation.",
                visual: guardrails_svg.name,
                description: "PII, Confidential Data, Incorrect outputs and more...",
                primary_action_drawer: Some(("Requires an Enterprise Licence".to_string(), "create-licence".to_string()))
            }
        }
    }
}
