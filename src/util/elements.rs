use thirtyfour::{By, WebDriver, error::WebDriverError, extensions::query::ElementQueryable};

pub async fn wait_until_element_gone(driver: &WebDriver, query: By) -> Result<(), WebDriverError> {
    driver.query(query).wait_until_gone().await
}
