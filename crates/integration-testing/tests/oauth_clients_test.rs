pub mod common;

use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_oauth_clients() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { oauth_clients(driver, &config).await })
    })
    .await
}

async fn oauth_clients(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    println!("Testing : register_user");

    let _email = common::register_user(driver, config).await?;

    test_oauth_clients(driver).await?;

    Ok(())
}

async fn test_oauth_clients(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("OAuth Clients")).await?;

    common::click_when_visible(driver, By::LinkText("Add OAuth Client")).await?;

    common::set_input(driver, By::Css("input[name='provider']"), "test-oauth").await?;
    common::set_input(
        driver,
        By::Css("input[name='provider_url']"),
        "https://example.com/oauth-test",
    )
    .await?;
    common::set_input(driver, By::Css("input[name='client_id']"), "client-id-123").await?;
    common::set_input(
        driver,
        By::Css("input[name='client_secret']"),
        "client-secret-456",
    )
    .await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Create OAuth Client']")).await?;

    common::wait_visible(driver, By::XPath("//h2[normalize-space()='test-oauth']")).await?;
    common::wait_visible(
        driver,
        By::XPath("//span[contains(normalize-space(), 'Client ID: client-id-123')]"),
    )
    .await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Delete']")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Delete OAuth Client']")).await?;

    let matches = driver
        .find_all(By::XPath("//h2[normalize-space()='test-oauth']"))
        .await?;
    assert!(matches.is_empty());

    Ok(())
}
