use thirtyfour::{By, WebDriver, error::WebDriverError, extensions::query::ElementQueryable};

use crate::config::ConfigCredentials;

const EMAIL_INPUT: &str = "input[type=\"email\"]";
const PASSWORD_INPUT: &str = "input[type=\"password\"]";
const LOGIN_PAGE: &str = "https://linkedin.com/login/";

pub async fn authenticate_in_driver(
    driver: &WebDriver,
    credentials: &ConfigCredentials,
) -> anyhow::Result<()> {
    driver.goto(LOGIN_PAGE).await?;

    send_to_input(driver, EMAIL_INPUT, &credentials.email, "el. pasto ivestis").await?;
    send_to_input(
        driver,
        PASSWORD_INPUT,
        &credentials.password,
        "slaptazodzio ivestis",
    )
    .await?;

    Ok(())
}

async fn send_to_input(
    driver: &WebDriver,
    query: &str,
    value: &str,
    desc: &str,
) -> anyhow::Result<(), WebDriverError> {
    let input = driver
        .query(By::Css(query))
        .and_displayed()
        .desc(desc)
        .first()
        .await?;

    println!("Input: {:?}", input.id().await?);
    input.send_keys(value).await
}
