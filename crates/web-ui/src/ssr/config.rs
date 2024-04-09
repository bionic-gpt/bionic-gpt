use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    // What's the maximum MB's of files we can upload in one go.
    pub max_upload_size_mb: usize,
    pub port: u16,
    // The gRPC server
    pub app_database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let port: u16 = if env::var("PORT").is_ok() {
            env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7703
        };

        let max_upload_size_mb: usize = if env::var("MAX_UPLOAD_SIZE_MB").is_ok() {
            env::var("MAX_UPLOAD_SIZE_MB")
                .unwrap()
                .parse::<usize>()
                .unwrap()
        } else {
            1000
        };

        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        Config {
            max_upload_size_mb,
            port,
            app_database_url,
        }
    }
}
