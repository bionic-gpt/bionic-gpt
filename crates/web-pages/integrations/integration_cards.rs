#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use std::collections::BTreeMap;

use super::IntegrationOas3;

// Default placeholder SVG for integrations without logos
const DEFAULT_INTEGRATION_LOGO: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHZpZXdCb3g9IjAgMCA0OCA0OCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjQ4IiBoZWlnaHQ9IjQ4IiByeD0iOCIgZmlsbD0iIzZCNzI4MCIvPgo8cGF0aCBkPSJNMTYgMTZIMzJWMjBIMTZWMTZaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNMTYgMjRIMzJWMjhIMTZWMjRaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNMTYgMzJIMjhWMzZIMTZWMzJaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4K";

/// Safely extracts the logo URL from integration extensions
fn get_logo_url(extensions: &BTreeMap<String, serde_json::Value>) -> String {
    extensions
        .get("logo")
        .and_then(|logo| logo.as_object())
        .and_then(|logo_obj| logo_obj.get("url"))
        .and_then(|url| url.as_str())
        .filter(|url| !url.is_empty())
        .map(|url| url.to_string())
        .unwrap_or_else(|| DEFAULT_INTEGRATION_LOGO.to_string())
}

#[component]
pub fn IntegrationCards(integrations: Vec<IntegrationOas3>, team_id: i32) -> Element {
    rsx!(
        h1 {
            "Integrations"
        }
        p {
            "Connect external tools to retrieve data, take actions, and more."
        }
        for integration in integrations.iter() {
            a {
                href: crate::routes::integrations::View {team_id, id: integration.integration.id }.to_string(),
                class: "no-underline",
                Card {
                    class: "p-3 mt-5 flex flex-row clickable",
                    img {
                        src: get_logo_url(&integration.spec.info.extensions),
                        width: "48",
                        height: "48"
                    }
                    div {
                        class: "ml-4",
                        h2 {
                            "{integration.spec.info.title.clone()}"
                        }
                        p {
                            if let Some(description) = integration.spec.info.description.clone() {
                                "{description}"
                            }
                        }
                    }
                }
            }
        }
    )
}
