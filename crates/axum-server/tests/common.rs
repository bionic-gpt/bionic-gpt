use db::Pool;
use rand::Rng;
use std::env;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct Config {
    pub webdriver_url: String,
    pub host: String,
    // The database
    pub db_pool: Pool,
    pub headless: bool,
    pub mailhog_url: String,
}

impl Config {
    pub async fn new() -> Config {
        let webdriver_url: String = if env::var("WEB_DRIVER_URL").is_ok() {
            env::var("WEB_DRIVER_URL").unwrap()
        } else {
            // Default to selenium in our dev container
            "http://selenium:4444".into()
        };

        let headless = env::var("ENABLE_HEADLESS").is_ok();

        let host = if env::var("WEB_DRIVER_DESTINATION_HOST").is_ok() {
            env::var("WEB_DRIVER_DESTINATION_HOST").unwrap()
        } else {
            "http://envoy:7700".into()
        };

        let mailhog_url = if env::var("MAILHOG_URL").is_ok() {
            env::var("MAILHOG_URL").unwrap()
        } else {
            "http://smtp:8025/api/v2/messages?limit=1".into()
        };

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let db_pool = db::create_pool(&database_url);

        Config {
            webdriver_url,
            host,
            db_pool,
            headless,
            mailhog_url,
        }
    }

    pub async fn get_driver(&self) -> WebDriverResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("--start-maximized")?;
        // We need the below otherwise window.crypto.subtle is not defined
        caps.add_chrome_arg("--unsafely-treat-insecure-origin-as-secure=http://envoy:7100")?;

        if self.headless {
            caps.set_headless()?;
        }
        WebDriver::new(&self.webdriver_url, caps).await
    }
}

pub async fn register_user(driver: &WebDriver, config: &Config) -> WebDriverResult<String> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .goto(format!("{}/auth/sign_up", &config.host))
        .await?;

    let email = register_random_user(driver).await?;

    Ok(email)
}

pub async fn register_random_user(driver: &WebDriver) -> WebDriverResult<String> {
    let email = random_email();

    // Register someone
    driver
        .find(By::Id("email"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find(By::Id("password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find(By::Id("confirm_password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    // OTP Code
    // Wait for page to load as code might not be in database yet.
    driver.find(By::Id("console-panel")).await?;

    Ok(email)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn random_email() -> String {
    let mut rng = rand::thread_rng();
    format!("{}@test.com", rng.gen::<u32>())
}
