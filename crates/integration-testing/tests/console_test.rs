pub mod common;

use thirtyfour::{components::SelectElement, prelude::*};
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_single_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { single_user(driver, &config).await })
    })
    .await
}

async fn single_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver
        .goto(format!("{}/auth/sign_up", &config.application_url))
        .await?;

    println!("Testing : register_user");

    let email = common::register_user(driver, config).await?;

    driver.refresh().await?;

    audit_filter(driver, &email).await?;

    test_console(driver).await?;

    Ok(())
}

async fn audit_filter(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Audit Trail")).await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    common::click_when_visible(driver, By::XPath("//button[text()='Filter']")).await?;

    let user_selector = common::wait_visible(driver, By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&user_selector).await?;
    select.select_by_exact_text(email).await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Apply Filter']")).await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // See it in the search results
    let table_cell = driver.find(By::XPath("//tbody/tr[last()]/td[2]")).await?;

    assert_eq!(table_cell.text().await?, email);

    Ok(())
}

async fn test_console(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Chat")).await?;

    common::set_input(driver, By::Css("textarea[name='message']"), "How are you?").await?;

    let delay = std::time::Duration::new(30, 0);
    driver.set_implicit_wait_timeout(delay).await?;
    common::click_when_visible(driver, By::XPath("//button[@id='prompt-submit-button']")).await?;

    common::wait_visible(driver, By::XPath("//a[text()='View Prompt']")).await?;

    Ok(())
}
