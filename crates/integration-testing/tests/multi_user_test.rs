pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_multi_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = multi_user(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn multi_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    let team_member = common::register_user(driver, config).await?;

    common::logout(driver, config).await?;

    let account_owner = common::register_user(driver, config).await?;

    println!("Testing : set_profile_details");

    set_profile_details(driver, &account_owner).await?;

    println!("Testing : add_team_member");

    add_team_member(driver, &team_member, &account_owner, config).await?;

    println!("Testing : sign_in_user");

    common::logout(driver, config).await?;

    common::sign_in_user(driver, &account_owner, config).await?;

    Ok(())
}

// Before we ivite people we have to have a team name and set our own name
async fn set_profile_details(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Click on the profile button
    let path = format!("//span[text()='{}']", email);
    driver.find(By::XPath(&path)).await?.click().await?;

    driver.find(By::LinkText("Profile")).await?.click().await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find(By::Css("input[name='first_name']"))
        .await?
        .send_keys("David")
        .await?;

    driver
        .find(By::Css("input[name='last_name']"))
        .await?
        .send_keys("Jason")
        .await?;

    driver
        .find(By::XPath("//button[text()='Update Profile']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Now set the org name
    driver
        .find(By::LinkText("Team Members"))
        .await?
        .click()
        .await?;

    driver
        .find(By::XPath("//button[text()='Edit Name']"))
        .await?
        .click()
        .await?;

    // Wait for the form to appear
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
        .send_keys("Testing Team")
        .await?;

    driver
        .find(By::XPath("//button[text()='Set Team Name']"))
        .await?
        .click()
        .await?;

    Ok(())
}

async fn add_team_member(
    driver: &WebDriver,
    team_member: &str,
    team_owner: &str,
    config: &common::Config,
) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Click on the side menu
    driver
        .find(By::LinkText("Team Members"))
        .await?
        .click()
        .await?;

    sleep(Duration::from_millis(3000)).await;

    driver.refresh().await?;

    sleep(Duration::from_millis(1000)).await;

    driver
        .find(By::XPath("//button[text()='Invite New Team Member']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find(By::Css("input[name='email']"))
        .await?
        .send_keys(team_member)
        .await?;

    driver
        .find(By::Css("input[name='first_name']"))
        .await?
        .send_keys("Trevor")
        .await?;

    driver
        .find(By::Css("input[name='last_name']"))
        .await?
        .send_keys("Invitable")
        .await?;

    driver
        .find(By::XPath("//button[text()='Send Invitation']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let table_cell = driver
        .find(By::XPath("//tbody/tr[last()]/td[1]/span"))
        .await?;

    assert_eq!(table_cell.text().await?, "Trevor Invitable");

    // Get the invite from mailhog
    let invitation_url = get_invite_url_from_email(config).await?;

    println!("Got the invite from mailhog");

    common::logout(driver, config).await?;

    common::sign_in_user(driver, team_member, config).await?;

    // Accept the invitation
    driver.goto(invitation_url).await?;

    let table_cell = driver.find(By::XPath("//tbody/tr[1]/td[2]")).await?;

    assert_eq!(table_cell.text().await?, team_owner);

    Ok(())
}

async fn get_invite_url_from_email(config: &common::Config) -> WebDriverResult<String> {
    let url = format!("{}/api/v2/messages?limit=1", config.mailhog_url);
    let body: String = reqwest::get(url).await.unwrap().text().await.unwrap();

    let url: Vec<&str> = body.split("Click ").collect();
    let url: Vec<&str> = url[1].split(' ').collect();

    // Emails are generally tructed to 78 columns. sigh.
    let url = quoted_printable::decode(url[0], quoted_printable::ParseMode::Robust).unwrap();
    let url = String::from_utf8(url).unwrap();

    let url = url.replace("\\u0026", "&");
    let url = url.replace("=\\r\\n", "");

    dbg!(&url);

    Ok(url)
}
