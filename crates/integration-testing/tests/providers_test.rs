pub mod common;

use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_providers() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { providers(driver, &config).await })
    })
    .await
}

async fn providers(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    println!("Testing : register_user");

    let _email = common::register_user(driver, config).await?;

    test_providers(driver).await?;

    Ok(())
}

async fn test_providers(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::LinkText("Providers")).await?;

    common::click_when_visible(driver, By::LinkText("Add Provider")).await?;

    common::set_input(driver, By::Css("input[name='name']"), "Test Provider").await?;
    common::set_input(
        driver,
        By::Css("input[name='base_url']"),
        "https://example.com",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("textarea[name='svg_logo']"),
        "<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24'></svg>",
    )
    .await?;

    common::set_input(
        driver,
        By::Css("input[name='default_model_name']"),
        "test-model",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("input[name='default_model_display_name']"),
        "Test Model",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("input[name='default_model_context_size']"),
        "2048",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("textarea[name='default_model_description']"),
        "Test model description",
    )
    .await?;

    common::set_input(
        driver,
        By::Css("input[name='default_embeddings_model_name']"),
        "test-embed",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("input[name='default_embeddings_model_display_name']"),
        "Test Embeddings",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("input[name='default_embeddings_model_context_size']"),
        "512",
    )
    .await?;
    common::set_input(
        driver,
        By::Css("textarea[name='default_embeddings_model_description']"),
        "Test embeddings description",
    )
    .await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Create Provider']")).await?;

    common::wait_visible(driver, By::XPath("//h2[normalize-space()='Test Provider']")).await?;

    // Ensure provider appears in Models -> Select Provider
    common::click_when_visible(driver, By::LinkText("Model Setup")).await?;
    common::click_when_visible(driver, By::LinkText("Add Model")).await?;
    common::wait_visible(driver, By::XPath("//h2[normalize-space()='Test Provider']")).await?;

    // Delete provider
    common::click_when_visible(driver, By::LinkText("Providers")).await?;
    common::click_when_visible(
        driver,
        By::XPath("//label[.//span[normalize-space()='...']]"),
    )
    .await?;
    common::click_when_visible(driver, By::LinkText("Delete")).await?;
    common::click_when_visible(driver, By::XPath("//button[text()='Delete']")).await?;

    let matches = driver
        .find_all(By::XPath("//h2[normalize-space()='Test Provider']"))
        .await?;
    assert!(matches.is_empty());

    Ok(())
}
