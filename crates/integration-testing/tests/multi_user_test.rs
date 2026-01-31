pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_multi_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    common::run_with_driver(&config, |driver| {
        let config = config.clone();
        Box::pin(async move { multi_user(driver, &config).await })
    })
    .await
}

async fn multi_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    let team_member = common::register_user(driver, config).await?;

    common::logout(driver, config).await?;

    let account_owner = common::register_user(driver, config).await?;

    println!("Testing : set_profile_details");

    set_profile_details(driver).await?;

    println!("Testing : add_team_member");

    add_team_member(driver, &team_member, config).await?;

    println!("Testing : sign_in_user");

    common::logout(driver, config).await?;

    common::sign_in_user(driver, &account_owner, config).await?;

    Ok(())
}

// Before we ivite people we have to have a team name and set our own name
async fn set_profile_details(driver: &WebDriver) -> WebDriverResult<()> {
    common::click_when_visible(driver, By::XPath("//span[text()='Test User']")).await?;

    common::click_when_visible(driver, By::LinkText("Profile")).await?;

    common::set_input(driver, By::Css("input[name='first_name']"), "David").await?;
    common::set_input(driver, By::Css("input[name='last_name']"), "Jason").await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Update Profile']")).await?;

    sleep(Duration::from_millis(3000)).await;

    common::click_when_visible(driver, By::LinkText("Admin Panel")).await?;

    common::click_when_visible(driver, By::LinkText("Teams")).await?;

    common::click_when_visible(
        driver,
        By::XPath("//div[contains(@class,'card') and @data-clickable-link]"),
    )
    .await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Edit Name']")).await?;

    common::set_input(driver, By::Css("input[name='name']"), "Testing Team").await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Set Team Name']")).await?;

    Ok(())
}

async fn add_team_member(
    driver: &WebDriver,
    team_member: &str,
    config: &common::Config,
) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    common::click_when_visible(driver, By::LinkText("Teams")).await?;

    common::click_when_visible(
        driver,
        By::XPath("//div[contains(@class,'card') and @data-clickable-link]"),
    )
    .await?;

    common::click_when_visible(
        driver,
        By::XPath("//button[text()='Invite New Team Member']"),
    )
    .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    common::set_input(driver, By::Css("input[name='email']"), team_member).await?;
    common::set_input(driver, By::Css("input[name='first_name']"), "Trevor").await?;
    common::set_input(driver, By::Css("input[name='last_name']"), "Invitable").await?;

    common::click_when_visible(driver, By::XPath("//button[text()='Send Invitation']")).await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    println!("Testing : Checking for Trevor");

    common::wait_visible(
        driver,
        By::XPath("//h2[normalize-space()='Trevor Invitable']"),
    )
    .await?;

    // Get the invite from mailhog
    let invitation_url = get_invite_url_from_email(config).await?;

    println!("Got the invite from mailhog");

    common::logout(driver, config).await?;

    common::sign_in_user(driver, team_member, config).await?;

    // Accept the invitation
    driver.goto(invitation_url).await?;

    let team_heading =
        common::wait_visible(driver, By::XPath("//h2[normalize-space()='Testing Team']")).await?;

    assert_eq!(team_heading.text().await?, "Testing Team");

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
