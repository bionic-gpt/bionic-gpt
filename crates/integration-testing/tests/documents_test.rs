pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_documents() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    driver.goto(format!("{}/", &config.application_url)).await?;

    println!("Testing : register_user");

    let _email = common::register_user(&driver, &config).await?;

    test_documents(&driver).await?;

    test_pipelines(&driver).await?;

    driver.quit().await?;

    Ok(())
}

async fn test_pipelines(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Document Pipelines"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Pipeline']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Pipeline']"))
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

async fn test_documents(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Datasets & Documents"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath(
            "//*[self::a or self::button][normalize-space()='Add Dataset']",
        ))
        .await?
        .click()
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
        .find(By::XPath("//button[text()='Add Document']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath(".//*[@type='file']"))
        .await?
        .send_keys("/home/seluser/workspace/files/parliamentary-dialog.txt")
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
