use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Integration {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub logo_url: String,
    pub logo_data_uri: String,
    pub source_url: String,
    pub filename: String,
}

pub fn catalogue() -> Vec<Integration> {
    serde_json::from_str(include_str!(concat!(
        env!("OUT_DIR"),
        "/integrations-catalog.json"
    )))
    .expect("failed to read generated integrations catalogue")
}
