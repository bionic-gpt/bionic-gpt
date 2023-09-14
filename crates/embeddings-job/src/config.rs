use std::env;

pub struct Config {
    pub app_database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        Config { app_database_url }
    }
}
