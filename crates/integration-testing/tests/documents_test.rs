pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_documents() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move {
            driver.goto(format!("{}/", &config.application_url)).await?;

            println!("Testing : register_user");

            let _email = common::register_user(driver, &config).await?;

            test_documents(driver).await?;

            test_pipelines(driver).await?;

            Ok(())
        })
    })
    .await
}

async fn test_pipelines(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Document Pipelines")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='New Pipeline']")).await?;
    common::set_input(driver, By::Css("input[name='name']"), "My Pipeline").await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Create Pipeline']")).await?;
    common::click_when_visible(driver, By::XPath("//td[text()='My Pipeline']")).await?;

    Ok(())
}

async fn test_documents(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Datasets & Documents")).await?;
    common::click_when_visible(
        driver,
        By::XPath("//*[self::a or self::button][normalize-space()='Add Dataset']"),
    )
    .await?;
    common::set_input(driver, By::Css("input[name='name']"), "Team Dataset").await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Save']")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Add Document']")).await?;

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

    common::wait_visible(
        driver,
        By::XPath("//*[contains(@class,'badge')][normalize-space()='Processed']"),
    )
    .await?;

    common::click_when_visible(
        driver,
        By::XPath("//label[.//span[normalize-space()='...']]"),
    )
    .await?;

    common::click_when_visible(driver, By::LinkText("Delete Document")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Delete Document']")).await?;

    common::wait_visible(
        driver,
        By::XPath(
            "//p[normalize-space()=\"This dataset doesn't have any documents yet. Upload your first document to get started.\"]",
        ),
    )
    .await?;

    Ok(())
}
