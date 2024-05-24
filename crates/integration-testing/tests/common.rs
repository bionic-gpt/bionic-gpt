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
        let host = if env::var("HOST_IP_ADDRESS").is_ok() {
            env::var("HOST_IP_ADDRESS").unwrap()
        } else {
            "localhost".into()
        };

        let webdriver_url = format!("http://{}:4444", host);

        let headless = env::var("ENABLE_HEADLESS").is_ok();

        let mailhog_url = format!("http://{}:8025/api/v2/messages?limit=1", host);

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let db_pool = db::create_pool(&database_url);

        let host = format!("http://{}", host);

        dbg!(&host);

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

    pub async fn set_sys_admin(&self, email: &str) -> WebDriverResult<()> {
        let client = self
            .db_pool
            .get()
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        client
            .execute(
                "UPDATE users SET system_admin = TRUE WHERE email = $1",
                &[&email],
            )
            .await
            .map_err(|e| WebDriverError::CustomError(e.to_string()))?;

        Ok(())
    }
}

pub async fn register_user(driver: &WebDriver, config: &Config) -> WebDriverResult<String> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver.goto(format!("{}/", &config.host)).await?;

    let email = register_random_user(driver).await?;

    Ok(email)
}

pub async fn logout(driver: &WebDriver) -> WebDriverResult<()> {
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

pub async fn register_random_user(driver: &WebDriver) -> WebDriverResult<String> {
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

    // OTP Code
    // Wait for page to load as code might not be in database yet.
    driver
        .find(By::Id("console-panel"))
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
    let mut rng = rand::thread_rng();
    format!("{}@test.com", rng.gen::<u32>())
}
