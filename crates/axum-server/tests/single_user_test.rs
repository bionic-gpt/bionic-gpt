pub mod common;

use thirtyfour::prelude::*;

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
        .goto(format!("{}/auth/sign_up", &config.host))
        .await?;

    println!("Testing : register_user");

    let _email = common::register_user(driver, config).await?;

    test_documents(driver).await?;

    test_prompts(driver).await?;

    test_console(driver).await?;

    Ok(())
}

async fn test_console(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Chat Console"))
        .await?
        .click()
        .await?;

    driver
        .query(By::Css("textarea[name='message']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

async fn test_prompts(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Prompt Engineering"))
        .await?
        .click()
        .await?;

    driver
        .query(By::LinkText("New Prompt Template"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .query(By::LinkText("New Prompt Template"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::LinkText("New Prompt Template"))
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
        .send_keys("My Prompt")
        .await?;

    driver
        .find(By::XPath("//button[text()='Submit']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//tr//td[contains(text(), 'My Prompt')]"))
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

    driver.find(By::LinkText("Edit")).await?.click().await?;

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
        .send_keys("My Prompt2")
        .await?;

    driver
        .find(By::XPath("//button[text()='Submit']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//tr//td[contains(text(), 'My Prompt2')]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

async fn test_documents(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Team Documents"))
        .await?
        .click()
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
        .find(By::XPath("//button[text()='Create Dataset']"))
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
        .find(By::XPath("//footer//button[text()='Upload File']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//button[contains(@class, 'Label--success')]"))
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
