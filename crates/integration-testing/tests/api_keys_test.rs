pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_api_keys() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { api_keys(driver, &config).await })
    })
    .await
}

async fn api_keys(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    println!("Testing : register_user");

    let _email = common::register_user(driver, config).await?;

    test_ai_assistants(driver).await?;

    test_api_keys(driver, config).await?;

    Ok(())
}

async fn test_ai_assistants(driver: &WebDriver) -> WebDriverResult<()> {
    sleep(Duration::from_millis(3000)).await;

    common::click_when_visible(driver, By::LinkText("Back to app")).await?;
    common::click_when_visible(driver, By::LinkText("Explore Assistants")).await?;

    common::click_when_visible(
        driver,
        By::XPath("//*[self::a or self::button][normalize-space()='New Assistant']"),
    )
    .await?;

    common::set_input(
        driver,
        By::XPath("(//input[@name='name'])[last()]"),
        "My Prompt",
    )
    .await?;
    common::set_input(
        driver,
        By::XPath("(//textarea[@name='description'])[last()]"),
        "A small test assistant. I don't do much.",
    )
    .await?;

    common::click_when_visible(
        driver,
        By::XPath("(//*[self::a or self::button][normalize-space()='Create Assistant'])[last()]"),
    )
    .await?;

    common::wait_visible(
        driver,
        By::XPath("//h2[contains(normalize-space(), 'My Prompt')]"),
    )
    .await?;

    common::click_when_visible(
        driver,
        By::XPath("//label[.//span[normalize-space()='...']]"),
    )
    .await?;

    common::click_when_visible(driver, By::LinkText("Edit")).await?;

    let name_input = common::wait_visible(driver, By::XPath("(//input[@name='name'])[1]")).await?;
    name_input.send_keys("2").await?;

    common::click_when_visible(
        driver,
        By::XPath("(//*[self::a or self::button][normalize-space()='Update Assistant'])[1]"),
    )
    .await?;

    common::wait_visible(
        driver,
        By::XPath("//h2[contains(normalize-space(), 'My Prompt2')]"),
    )
    .await?;

    Ok(())
}

async fn test_api_keys(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Admin Panel")).await?;
    common::click_when_visible(driver, By::LinkText("API Keys")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Create Assistant Key']"))
        .await?;

    common::set_input(driver, By::Css("input[name='name']"), "Test Key").await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Create API Key']")).await?;

    common::wait_visible(driver, By::XPath("//td[text()='Test Key']")).await?;

    let api_key_input = driver
        .find(By::XPath("//input[@name='generated-api-key']"))
        .await?;

    let api_key = api_key_input.value().await?.unwrap();

    let client = reqwest::Client::new();

    println!(
        "curl -X GET '{}/v1/models' -H 'Authorization: Bearer {}'",
        &config.api_base_url, api_key
    );

    // Making a GET request and passing the API key in the headers
    let response = client
        .get(format!("{}/v1/models", &config.api_base_url))
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
