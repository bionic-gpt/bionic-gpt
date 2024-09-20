use std::env;

#[derive(Debug)]
pub struct Config {
    pub app_database_url: String,
    pub unstructured_endpoint: String,
}

impl Config {
    pub fn new() -> Config {
        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        let unstructured_endpoint = if let Ok(domain) = std::env::var("CHUNKING_ENGINE") {
            domain
        } else {
            "http://chunking-engine:8000".to_string()
        };

        Config {
            app_database_url,
            unstructured_endpoint,
        }
    }
}
