pub mod common;

use thirtyfour::{components::SelectElement, prelude::*};
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_single_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = single_user(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn single_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver
        .goto(format!("{}/auth/sign_up", &config.application_url))
        .await?;

    println!("Testing : register_user");

    let email = common::register_user(driver, config).await?;
    config.set_sys_admin(&email).await?;

    driver.refresh().await?;

    audit_filter(driver, &email).await?;

    test_console(driver).await?;

    Ok(())
}

async fn audit_filter(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    let audit_link = driver.find(By::LinkText("Audit Trail")).await?;
    audit_link.click().await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let filter_button = driver.find(By::XPath("//button[text()='Filter']")).await?;
    filter_button.click().await?;

    driver
        .find(By::Css("select:first-of-type"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    let user_selector = driver.find(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&user_selector).await?;
    select.select_by_exact_text(email).await?;

    driver
        .find(By::XPath("//button[text()='Apply Filter']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // See it in the search results
    let table_cell = driver.find(By::XPath("//tbody/tr[last()]/td[2]")).await?;

    assert_eq!(table_cell.text().await?, email);

    Ok(())
}

async fn test_console(driver: &WebDriver) -> WebDriverResult<()> {
    driver.find(By::LinkText("Chat")).await?.click().await?;

    driver
        .query(By::Css("textarea[name='message']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::Css("textarea[name='message']"))
        .await?
        .send_keys("How are you?")
        .await?;

    let delay = std::time::Duration::new(30, 0);
    driver.set_implicit_wait_timeout(delay).await?;
    driver
        .find(By::XPath("//button[@id='prompt-submit-button']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//a[text()='View Prompt']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}
