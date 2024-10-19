pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_documents() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = documents(&driver, &config).await;

    test_pipelines(&driver).await?;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn test_pipelines(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Data Integrations"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Document Pipeline']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Document Pipeline']"))
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
        .send_keys("My Pipeline")
        .await?;

    driver
        .find(By::XPath("//button[text()='Create Pipeline']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//td[text()='My Pipeline']"))
        .await?
        .click()
        .await?;

    Ok(())
}

async fn documents(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver
        .goto(format!("{}/auth/sign_up", &config.application_url))
        .await?;

    println!("Testing : register_user");

    let email = common::register_user(driver, config).await?;
    config.set_sys_admin(&email).await?;

    driver.refresh().await?;

    test_documents(driver).await?;

    Ok(())
}

async fn test_documents(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Datasets & Documents"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='Add Dataset']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='Add Dataset']"))
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
        .send_keys("Team Dataset")
        .await?;

    driver
        .find(By::XPath("//button[text()='Save']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//button[text()='Add Document']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='Add Document']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath(".//*[@type='file']"))
        .await?
        .send_keys("/workspace/parliamentary-dialog.txt")
        .await?;

    driver
        .find(By::XPath("//button[text()='Upload File(s)']"))
        .await?
        .click()
        .await?;

    // Wait for file to upload then refresh the page
    // Otherwise we don't see the status
    sleep(Duration::from_millis(10000)).await;
    driver.refresh().await?;

    driver
        .query(By::XPath("//button[contains(@class, 'label-success')]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//span[text()='...']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::LinkText("Delete Document"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//button[text()='Delete Document']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='Delete Document']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath(
            "//p[text()='Here you can upload documents in a range of formats']",
        ))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}
