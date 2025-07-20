use std::env;

#[derive(Debug)]
pub struct Config {
    pub app_database_url: String,
    pub unstructured_endpoint: String,
    pub batch_size: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        let unstructured_endpoint = if let Ok(domain) = std::env::var("CHUNKING_ENGINE") {
            domain
        } else {
            "http://chunking-engine:8000".to_string()
        };

        let batch_size = std::env::var("RAG_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        Config {
            app_database_url,
            unstructured_endpoint,
            batch_size,
        }
    }
}
