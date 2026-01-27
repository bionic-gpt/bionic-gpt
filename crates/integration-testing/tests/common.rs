use db::Pool;
use rand::Rng;
use std::env;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct Config {
    pub webdriver_url: String,
    pub application_url: String,
    // The database
    pub db_pool: Pool,
    pub headless: bool,
    pub mailhog_url: String,
    pub api_base_url: String,
}

impl Config {
    pub async fn new() -> Config {
        let webdriver_url = if env::var("WEB_DRIVER_URL").is_ok() {
            env::var("WEB_DRIVER_URL").unwrap()
        } else {
            "http://localhost:4444".into()
        };

        let application_url = if env::var("APPLICATION_URL").is_ok() {
            env::var("APPLICATION_URL").unwrap()
        } else {
            "http://nginx:80".into()
        };

        let mailhog_url = if env::var("MAILHOG_URL").is_ok() {
            env::var("MAILHOG_URL").unwrap()
        } else {
            "http://localhost:8025".into()
        };

        let api_base_url = if env::var("API_BASE_URL").is_ok() {
            env::var("API_BASE_URL").unwrap()
        } else {
            "http://localhost:7901".into()
        };

        let headless = env::var("ENABLE_HEADLESS").is_ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let db_pool = db::create_pool(&database_url);

        dbg!(&webdriver_url);
        dbg!(&application_url);
        dbg!(&mailhog_url);
        dbg!(&database_url);
        dbg!(&api_base_url);

        Config {
            webdriver_url,
            application_url,
            db_pool,
            headless,
            mailhog_url,
            api_base_url,
        }
    }

    pub async fn get_driver(&self) -> WebDriverResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("--start-maximized")?;
        caps.add_chrome_arg("--ignore-certificate-errors")?;

        if self.headless {
            caps.set_headless()?;
        }
        WebDriver::new(&self.webdriver_url, caps).await
    }

    pub async fn remove_users(&self) -> WebDriverResult<()> {
        let mut client = self
            .db_pool
            .get()
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        dbg!("connected");

        let transaction = client
            .transaction()
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        transaction
            .execute("SET LOCAL session_replication_role = replica", &[])
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        transaction
            .execute("DELETE FROM models", &[])
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        transaction
            .execute("DELETE FROM users", &[])
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        Ok(())
    }
}

pub async fn register_user(driver: &WebDriver, config: &Config) -> WebDriverResult<String> {
    // We have to remove users as only the first created user gets auto set to sysadmin
    config.remove_users().await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver.goto(format!("{}/", &config.application_url)).await?;

    let email = register_random_user(driver, config).await?;

    Ok(email)
}

pub async fn sign_in_user(driver: &WebDriver, email: &str, config: &Config) -> WebDriverResult<()> {
    // Go to sign in page
    driver.goto(format!("{}/", &config.application_url)).await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Sign in someone
    driver
        .find(By::Id("username"))
        .await?
        .send_keys(email)
        .await?;
    driver
        .find(By::Id("password"))
        .await?
        .send_keys(email)
        .await?;
    driver
        .find(By::Css("input[type='submit']"))
        .await?
        .click()
        .await?;

    Ok(())
}

pub async fn logout(driver: &WebDriver, _config: &Config) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find(By::Css("div.dropdown.dropdown-top"))
        .await?
        .click()
        .await?;

    driver
        .find(By::LinkText("Log Out"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver.find(By::LinkText("Log Out")).await?.click().await?;

    driver
        .find(By::XPath("//button[text()='Logout']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='Logout']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::Id("kc-logout"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver.find(By::Id("kc-logout")).await?.click().await?;

    driver
        .find(By::Id("kc-header-wrapper"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

pub async fn register_random_user(driver: &WebDriver, _config: &Config) -> WebDriverResult<String> {
    let email = random_email();

    // Register someone
    driver.find(By::LinkText("Register")).await?.click().await?;

    driver
        .find(By::Id("firstName"))
        .await?
        .send_keys("Test")
        .await?;

    driver
        .find(By::Id("lastName"))
        .await?
        .send_keys("User")
        .await?;

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
        .find(By::Id("password-confirm"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find(By::Css("input[type='submit']"))
        .await?
        .click()
        .await?;

    // Models setup (no models yet, so we land on select provider)
    let ollama_card = driver
        .find(By::XPath("//h2[normalize-space()='Ollama']"))
        .await?;
    ollama_card.wait_until().displayed().await?;
    ollama_card.click().await?;

    let api_key_input = driver
        .find(By::XPath(
            "//h3[contains(normalize-space(), 'Create Model: Ollama')]/ancestor::*[self::form or self::dialog]//input[@name='api_key']",
        ))
        .await?;
    api_key_input.wait_until().displayed().await?;
    api_key_input.send_keys("ollama-test-key").await?;

    driver
        .find(By::XPath(
            "//h3[contains(normalize-space(), 'Create Model: Ollama')]/ancestor::*[self::form or self::dialog]//button[normalize-space()='Create Model']",
        ))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath(
            "//*[self::a or self::button][contains(normalize-space(), 'Add Model')]",
        ))
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(email)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn random_email() -> String {
    let mut rng = rand::rng();
    format!("{}@test.com", rng.random::<u32>())
}
