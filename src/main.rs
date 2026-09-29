use log::error;
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::{
    auth::authenticate_in_driver,
    config::{create_logger, read_config},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = read_config();

    if let Err(e) = config {
        error!("Klaida nuskaitant konfiguraciją: {}", e);

        return Err(e);
    }

    let config = config.unwrap();

    create_logger(&config.logging);

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::managed(caps).await?;
    authenticate_in_driver(&driver, &config.credentials)
        .await
        .inspect_err(|e| error!("Klaida prisijungiant: {}", e))?;

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");

    driver.quit().await?;

    Ok(())
}

mod auth;
mod config;
mod util;
