use lettre::message;
use std::env;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    // Configure SMTP for email.
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub unstructured_endpoint: String,
    pub from_email: message::Mailbox,
}

impl SmtpConfig {
    pub fn new() -> Option<SmtpConfig> {
        let host = env::var("SMTP_HOST");
        let username = env::var("SMTP_USERNAME");
        let password = env::var("SMTP_PASSWORD");
        let smtp_port = env::var("SMTP_PORT");
        let domain = env::var("INVITE_DOMAIN");
        let from_email = env::var("INVITE_FROM_EMAIL_ADDRESS");
        let unstructured_endpoint = if let Ok(domain) = env::var("UNSTRUCTURED_ENDPOINT") {
            domain
        } else {
            "http://unstructured:8000".to_string()
        };

        if let (Ok(host), Ok(username), Ok(password), Ok(smtp_port), Ok(domain), Ok(from_email)) =
            (host, username, password, smtp_port, domain, from_email)
        {
            Some(SmtpConfig {
                host,
                port: smtp_port.parse::<u16>().unwrap(),
                tls_off: env::var("SMTP_TLS_OFF").is_ok(),
                username,
                password,
                domain,
                unstructured_endpoint,
                from_email: from_email.parse().unwrap(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    // What's the maximum MB's of files we can upload in one go.
    pub max_upload_size_mb: usize,
    pub port: u16,
    // The gRPC server
    pub app_database_url: String,
    // Configure SMTP for email.
    pub smtp_config: Option<SmtpConfig>,
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
            smtp_config: SmtpConfig::new(),
        }
    }
}
