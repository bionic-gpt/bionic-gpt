pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_openapi_specs() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { openapi_specs(driver, &config).await })
    })
    .await
}

async fn openapi_specs(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    println!("Testing : register_user");

    let _email = common::register_user(driver, config).await?;

    test_openapi_specs_crud(driver).await?;

    Ok(())
}

async fn test_openapi_specs_crud(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("OpenAPI Specs")).await?;

    common::click_when_visible(driver, By::LinkText("Add OpenAPI Spec")).await?;

    // Create a spec
    common::set_input(driver, By::Css("input[name='slug']"), "sample-spec").await?;
    common::set_input(driver, By::Css("input[name='title']"), "Sample Spec").await?;
    common::set_input(
        driver,
        By::Css("textarea[name='description']"),
        "Test spec created by integration test.",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("input[name='logo_url']"),
        "https://example.com/logo.svg",
    )
    .await?;

    common::set_input(
        driver,
        By::Css("textarea[name='spec']"),
        r#"{"openapi":"3.1.0","info":{"title":"Sample Spec","version":"1.0.0"}}"#,
    )
    .await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Create Spec']")).await?;

    common::wait_visible(driver, By::XPath("//td[text()='Sample Spec']")).await?;

    // Edit the spec
    common::click_when_visible(
        driver,
        By::XPath("//label[.//span[normalize-space()='...']]"),
    )
    .await?;
    common::click_when_visible(driver, By::LinkText("Edit")).await?;

    common::set_input(
        driver,
        By::Css("input[name='title']"),
        "Sample Spec Updated",
    )
    .await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Save Changes']")).await?;

    common::wait_visible(driver, By::XPath("//td[text()='Sample Spec Updated']")).await?;

    // Delete the spec
    common::click_when_visible(
        driver,
        By::XPath("//label[.//span[normalize-space()='...']]"),
    )
    .await?;
    common::click_when_visible(driver, By::LinkText("Delete")).await?;
    common::wait_visible(
        driver,
        By::XPath("//h3[normalize-space()='Delete this OpenAPI Spec?']"),
    )
    .await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Delete']")).await?;

    // Table should no longer contain the title
    wait_until_gone(driver, By::XPath("//td[text()='Sample Spec Updated']")).await?;

    // Invalid JSON path
    common::click_when_visible(driver, By::LinkText("Add OpenAPI Spec")).await?;
    common::set_input(driver, By::Css("input[name='slug']"), "bad-json").await?;
    common::set_input(driver, By::Css("input[name='title']"), "Bad JSON").await?;
    common::set_input(driver, By::Css("textarea[name='spec']"), "{invalid_json").await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Create Spec']")).await?;

    common::wait_visible(
        driver,
        By::XPath("//div[contains(@class,'alert')][contains(.,'Invalid JSON')]"),
    )
    .await?;

    Ok(())
}

async fn wait_until_gone(driver: &WebDriver, by: By) -> WebDriverResult<()> {
    for _ in 0..10 {
        let matches = driver.find_all(by.clone()).await?;
        if matches.is_empty() {
            return Ok(());
        }
        sleep(Duration::from_millis(500)).await;
    }
    let matches = driver.find_all(by).await?;
    assert!(matches.is_empty());
    Ok(())
}
