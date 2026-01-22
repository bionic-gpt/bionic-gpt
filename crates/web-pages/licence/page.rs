#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::card_item::CardItem;

use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, customer_keys, Licence};
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, callback_url: String, version: String) -> String {
    let licence = Licence::global();
    let encryption = if customer_keys::get_customer_key().is_some() {
        "Enabled"
    } else {
        "Disabled"
    };
    let automations = if licence.features.automations {
        "Enabled"
    } else {
        "Disabled"
    };
    let mcp = if licence.features.mcp {
        "Enabled"
    } else {
        "Disabled"
    };
    let licence_logo = if licence.app_logo_svg.is_empty() {
        bionic_logo_svg.name.to_string()
    } else {
        format!("data:image/svg+xml;base64,{}", licence.app_logo_svg)
    };
    let app_name = if licence.app_name.is_empty() {
        "Bionic".to_string()
    } else {
        licence.app_name.clone()
    };
    let default_redirect_url = crate::routes::console::Index { team_id }.to_string();
    let redirect_url = licence
        .redirect_url
        .as_ref()
        .map(|template| template.replace("{team_id}", &team_id.to_string()))
        .filter(|url| !url.is_empty())
        .unwrap_or(default_redirect_url);

    let page = rsx! {
        AdminLayout {
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
                    image_src: Some(licence_logo),
                    avatar_name: None,
                    title: app_name.clone(),
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
                    image_src: Some(nav_automations_svg.name.to_string()),
                    avatar_name: None,
                    title: "Automations".to_string(),
                    description: Some(rsx!( span { "{automations}" } )),
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
                    title: "MCP".to_string(),
                    description: Some(rsx!( span { "{mcp}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_api_keys_svg.name.to_string()),
                    avatar_name: None,
                    title: "OAuth Callback URL".to_string(),
                    description: Some(rsx!( span { "{callback_url}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
                CardItem {
                    class: None,
                    popover_target: None,
                    clickable_link: None,
                    image_src: Some(nav_api_keys_svg.name.to_string()),
                    avatar_name: None,
                    title: "Redirect URL".to_string(),
                    description: Some(rsx!( span { "{redirect_url}" } )),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                }
            }
        }
    };
    crate::render(page)
}
