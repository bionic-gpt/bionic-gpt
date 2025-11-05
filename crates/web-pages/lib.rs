use db::Visibility;
use dioxus::prelude::Element;

pub mod api_keys;
pub mod app_layout;
pub mod assistants;
pub mod audit_trail;
pub mod automations;
pub mod categories;
pub mod charts;
pub mod components;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod history;
pub mod i18n;
pub mod integrations;
pub mod licence;
pub mod mcp_api_keys;
pub mod menu;
pub mod models;
pub mod my_assistants;
pub mod oauth_clients;
pub mod openapi_specs;
pub mod pipelines;
pub mod profile;
pub mod profile_popup;
pub mod rate_limits;
pub mod shared;
pub use components::section_introduction::SectionIntroduction;
pub mod snackbar;
pub mod team;
pub mod teams;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

pub mod routes;

pub fn visibility_to_string(visibility: Visibility) -> String {
    match visibility {
        Visibility::Private => "Private".to_string(),
        Visibility::Team => "Team".to_string(),
        Visibility::Company => "Everyone".to_string(),
    }
}

pub fn string_to_visibility(visibility: &str) -> Visibility {
    match visibility {
        "Team" => Visibility::Team,
        "Everyone" => Visibility::Company,
        _ => Visibility::Private,
    }
}
