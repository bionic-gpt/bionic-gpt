pub mod delete_drawer;
pub mod details_modal;
pub mod index;
use std::collections::BTreeMap;

use db::Integration;
pub mod integration_cards;
pub mod integration_type;
pub mod status_type;
pub mod upsert;
pub mod view;

#[derive(Clone, PartialEq, Debug)]
pub struct IntegrationOas3 {
    pub integration: Integration,
    pub spec: oas3::Spec,
}

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
