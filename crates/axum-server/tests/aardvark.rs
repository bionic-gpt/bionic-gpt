// File is called aardvark as we want it to run first.
pub mod common;

use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_multi_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    // If we have a EXTERNAL_API set then edit the base model
    // to use it.
    let result = sys_admin(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn sys_admin(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    let _sys_admin = common::register_user(driver, config).await?;

    Ok(())
}
