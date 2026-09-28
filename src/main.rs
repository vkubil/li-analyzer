use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::{auth::authenticate_in_driver, config::read_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = read_config();

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::managed(caps).await?;

    authenticate_in_driver(&driver, &config.credentials).await?;

    driver.quit().await?;

    Ok(())
}

mod auth;
mod config;
