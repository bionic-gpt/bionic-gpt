#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::card_item::CardItem;
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, customer_keys, Licence};
use dioxus::prelude::*;
use toml::Value;

fn get_version() -> String {
    let cargo_toml = include_str!("../../k8s-operator/Cargo.toml");
    let value: Value = cargo_toml.parse().expect("valid Cargo.toml");
    value
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}

pub fn page(team_id: i32, rbac: Rbac) -> String {
    let licence = Licence::global();
    let encryption = if customer_keys::get_customer_key().is_some() {
        "Enabled"
    } else {
        "Disabled"
    };
    let version = get_version();
    let rag_mode = if std::env::var("AGENTIC_RAG").is_ok() {
        "Agentic RAG"
    } else {
        "Contextual RAG"
    };
    let automations = if std::env::var("AUTOMATIONS_FEATURE").is_ok() {
        "Enabled"
    } else {
        "Disabled"
    };

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Licence,
            team_id: team_id,
            rbac: rbac,
            title: "System Info",
            header: rsx!(
                Breadcrumb { items: vec![BreadcrumbItem { text: "System Info".into(), href: None }] }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto space-y-4",
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(bionic_logo_svg.name.to_string()),
                    avatar_name: None,
                    title: "Licence".to_string(),
                    description: Some(rsx!( span { "Users: {licence.user_count}, Domain: {licence.hostname_url}, Expires: {licence.end_date.date()}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_audit_svg.name.to_string()),
                    avatar_name: None,
                    title: "Runtime Encryption".to_string(),
                    description: Some(rsx!( span { "{encryption}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_phonebook_svg.name.to_string()),
                    avatar_name: None,
                    title: "Version".to_string(),
                    description: Some(rsx!( span { "{version}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_service_requests_svg.name.to_string()),
                    avatar_name: None,
                    title: "RAG Mode".to_string(),
                    description: Some(rsx!( span { "{rag_mode}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_automations_svg.name.to_string()),
                    avatar_name: None,
                    title: "Automations".to_string(),
                    description: Some(rsx!( span { "{automations}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
            }
        }
    };
    crate::render(page)
}
