use thirtyfour::{DesiredCapabilities, WebDriver, error::WebDriverError};

pub async fn create_web_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    WebDriver::managed(caps).await
}
