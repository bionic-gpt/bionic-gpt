pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_api_keys() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = api_keys(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn api_keys(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver
        .goto(format!("{}/auth/sign_up", &config.application_url))
        .await?;

    println!("Testing : register_user");

    let email = common::register_user(driver, config).await?;
    config.set_sys_admin(&email).await?;

    driver.refresh().await?;

    test_ai_assistants(driver).await?;

    test_api_keys(driver, config).await?;

    Ok(())
}

async fn test_ai_assistants(driver: &WebDriver) -> WebDriverResult<()> {
    sleep(Duration::from_millis(3000)).await;

    driver.refresh().await?;

    driver
        .find(By::LinkText("Explore Assistants"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::LinkText("Explore Assistants"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Assistant']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Assistant']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("(//input[@name='name'])[last()]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("(//input[@name='name'])[last()]"))
        .await?
        .send_keys("My Prompt")
        .await?;

    driver
        .find(By::XPath("(//textarea[@name='description'])[last()]"))
        .await?
        .send_keys("A small test assistant. I don't do much.")
        .await?;

    driver
        .find(By::XPath("(//button[text()='Submit'])[last()]"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath(
            "//table//td//strong[contains(text(), 'My Prompt')]",
        ))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//table//td[.//button[text()='Edit']][1]"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("(//input[@name='name'])[1]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("(//input[@name='name'])[1]"))
        .await?
        .send_keys("2")
        .await?;

    driver
        .find(By::XPath("(//button[text()='Submit'])[1]"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath(
            "//table//td//strong[contains(text(), 'My Prompt2')]",
        ))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

async fn test_api_keys(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    driver.find(By::LinkText("API Keys")).await?.click().await?;

    driver
        .find(By::XPath("//button[text()='New API Key']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New API Key']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::Css("input[name='name']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::Css("input[name='name']"))
        .await?
        .send_keys("Test Key")
        .await?;

    driver
        .find(By::XPath("//button[text()='Create API Key']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//td[text()='Test Key']"))
        .await?
        .click()
        .await?;

    let api_key_input = driver.find(By::XPath("//input[@name='api_key']")).await?;

    let api_key = api_key_input.value().await?.unwrap();

    let client = reqwest::Client::new();

    println!(
        "curl -X GET 'http://localhost:7703/v1/models' -H 'Authorization: Bearer {}'",
        api_key
    );

    // Making a GET request and passing the API key in the headers
    let response = client
        .get(format!("{}/v1/models", &config.application_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    dbg!(&response);

    assert!(response.is_ok());

    if let Ok(response) = response {
        assert!(response.status().is_success());
        let text = response.text().await;
        assert!(text.is_ok());
        if let Ok(text) = text {
            assert!(text.contains("model"));
        }
    }

    // Make a request with no auth
    let response = client
        .get(format!("{}/v1/models", &config.application_url))
        .send()
        .await;

    dbg!(&response);

    assert!(response.is_ok());

    if let Ok(response) = response {
        let text = response.text().await;
        assert!(text.is_ok());
        if let Ok(text) = text {
            dbg!(&text);
            assert!(text.contains("You need an API key"));
        }
    }

    Ok(())
}
