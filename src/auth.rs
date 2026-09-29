use futures::future::join_all;
use log::{debug, warn};
use thirtyfour::{
    By, WebDriver, WebElement, error::WebDriverError, extensions::query::ElementQueryable,
};

use crate::{config::ConfigCredentials, util::elements::wait_until_element_gone};

const EMAIL_INPUT: &str = "input[type=\"email\"]";
const PASSWORD_INPUT: &str = "input[type=\"password\"]";
const SIGN_IN_BUTTON: &str = "button[type=\"button\"]";
const LOGIN_PAGE: &str = "https://linkedin.com/login/";
const SIGN_IN_BUTTON_TEXT: &str = "Sign in";

const CHECKPOINT_URL: &str = "/checkpoint/challengesV2";
const LOGIN_PAGE_ID: &str = "root";
const CHALLENGE_PAGE_ID: &str = "app__container";

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Nebuvo rastas prisijungimo mygtukas")]
    SignInButtonNotFound,
}

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

    debug!("Prisijungimo duomenys suvesti");

    // Nes sunku suzinoti, kuris is ju yra prisijungimo mygtukas, tai mums reikes perziureti visus mygtukus
    // ir rasti kuris is ju yra 'Sign in'
    let sign_ins = driver
        .query(By::Css(SIGN_IN_BUTTON))
        .and_displayed()
        .desc("prisijungimo mygtukas")
        .any()
        .await?;

    let sign_in = find_actual_sign_in(&sign_ins)
        .await
        .ok_or(LoginError::SignInButtonNotFound)
        .inspect_err(|e| log::error!("{}", e))?;

    debug!("Spaudžiamas prisijungimo mygtukas");
    sign_in.click().await?;

    debug!("Laukiama, kada pasikeis puslapis");

    // Nera kaip kitaip suzinoti, kada puslapis pasikeicia
    wait_until_element_gone(driver, By::Id(LOGIN_PAGE_ID)).await?;

    // Gauti URL po paspaudimo
    let url = driver.current_url().await?;
    debug!("Puslapis pasikeite. Naujas URL: {}", url);

    if url.path().starts_with(CHECKPOINT_URL) {
        warn!("Prašoma įvykdyti paskyros patvirtinimą per telefoninę programėlę.");
        // Kad nereiketu ir cia paspausti enter ar panasiai
        wait_until_element_gone(driver, By::Id(CHALLENGE_PAGE_ID)).await?;
    }

    Ok(())
}

pub async fn return_if_element(elem: &WebElement) -> Option<WebElement> {
    let text = elem.text().await.ok()?;

    if text == SIGN_IN_BUTTON_TEXT {
        Some(elem.clone())
    } else {
        None
    }
}

pub async fn find_actual_sign_in(sign_ins: &Vec<WebElement>) -> Option<WebElement> {
    let sign_in = join_all(sign_ins.iter().map(return_if_element)).await;

    sign_in.iter().find_map(|x| x.clone())
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

    input.send_keys(value).await
}
