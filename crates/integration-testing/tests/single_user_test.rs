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

    test_documents(driver).await?;

    test_console(driver).await?;

    test_prompts(driver).await?;

    test_api_keys(driver, config).await?;

    test_pipelines(driver).await?;

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

async fn test_api_keys(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("AI Assistant API"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='New API Key']"))
        .await?
        .wait_until()
        .displayed()
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

    let api_key_input = driver.find(By::XPath("//input[@name='api_key']")).await?;

    let api_key = api_key_input.value().await?.unwrap();

    let client = reqwest::Client::new();

    // Making a GET request and passing the API key in the headers
    let response = client
        .get(format!("{}/v1/models", &config.application_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

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

async fn test_console(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::LinkText("All Chats"))
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

    sleep(Duration::from_millis(3000)).await;

    driver.refresh().await?;

    driver
        .find(By::XPath("//button[text()='Show History']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}

async fn test_prompts(driver: &WebDriver) -> WebDriverResult<()> {
    sleep(Duration::from_millis(3000)).await;

    driver.refresh().await?;

    driver
        .find(By::LinkText("AI Assistants"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::LinkText("AI Assistants"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Assistant']"))
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find(By::XPath("//button[text()='New Assistant']"))
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

    driver
        .find(By::LinkText("Edit"))
        .await?
        .wait_until()
        .displayed()
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
