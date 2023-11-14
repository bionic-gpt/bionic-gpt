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

    test_console(driver).await?;

    test_prompts(driver).await?;

    test_api_keys(driver).await?;

    test_pipelines(driver).await?;

    Ok(())
}

async fn test_pipelines(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Document Pipelines"))
        .await?
        .click()
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

async fn test_api_keys(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("Chat API Keys"))
        .await?
        .click()
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

    driver
        .find(By::Css("textarea[name='message']"))
        .await?
        .send_keys("How are you?")
        .await?;

    let delay = std::time::Duration::new(30, 0);
    driver.set_implicit_wait_timeout(delay).await?;
    driver
        .find(By::XPath("//button[text()='Send Message']"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//a[text()='View Prompt']"))
        .await?
        .wait_until()
        .displayed()
        .await?;
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    // Test history popup
    driver
        .find(By::XPath("//button[text()='Show History']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='Show History']"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//h4[text()='Your History']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::LinkText("How are you?"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='Show History']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

async fn test_prompts(driver: &WebDriver) -> WebDriverResult<()> {
    driver.find(By::LinkText("Prompts")).await?.click().await?;

    driver
        .find(By::XPath("//button[text()='New Prompt']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Prompt']"))
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
        .find(By::XPath("(//button[text()='Submit'])[last()]"))
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
        .query(By::XPath("(//input[@name='name'])[1]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("(//input[@name='name'])[1]"))
        .await?
        .send_keys("My Prompt2")
        .await?;

    driver
        .find(By::XPath("(//button[text()='Submit'])[1]"))
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
        .find(By::LinkText("Team Datasets"))
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
