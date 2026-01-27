use std::env;

#[derive(Debug)]
pub struct Config {
    pub app_database_url: String,
    pub chunking_engine: ChunkingEngine,
    pub unstructured_endpoint: String,
    pub kreuzberg_endpoint: String,
    pub batch_size: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkingEngine {
    UnstructuredApi,
    KreuzbergApi,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        let chunking_engine = match env::var("CHUNKING_ENGINE") {
            Ok(value) if value.eq_ignore_ascii_case("UNSTRUCTURED_API") => {
                ChunkingEngine::UnstructuredApi
            }
            _ => ChunkingEngine::KreuzbergApi,
        };

        let unstructured_endpoint = env::var("UNSTRUCTURED_ENDPOINT")
            .ok()
            .unwrap_or_else(|| "http://chunking-engine:8000".to_string());

        let kreuzberg_endpoint = env::var("KREUZBERG_API_ENDPOINT")
            .ok()
            .unwrap_or_else(|| "http://doc-engine:8000".to_string());

        let batch_size = std::env::var("RAG_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        Config {
            app_database_url,
            chunking_engine,
            unstructured_endpoint,
            kreuzberg_endpoint,
            batch_size,
        }
    }
}
