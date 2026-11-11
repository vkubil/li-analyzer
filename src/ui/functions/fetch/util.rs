use std::sync::Arc;

use diesel::{RunQueryDsl, query_dsl::methods::SelectDsl};
use log::info;
use thirtyfour::{
    Cookie, WebDriver,
    error::{WebDriverError, WebDriverErrorInfo, WebDriverResult},
};

use crate::{
    database::{connection::establish_connection, schema::imported_graduates},
    web::{create::create_web_driver, extensions::WebDriverCookieExtension},
};

pub async fn create_driver_for_fetch(cookies: &Arc<Vec<Cookie>>) -> WebDriverResult<WebDriver> {
    let driver = create_web_driver().await?;

    info!("Kuriamas web driver");
    driver.clone().leak().map_err(|_| {
        WebDriverError::UnknownError(WebDriverErrorInfo::new(
            "Klaida leak'inant driverį užduočiai atlikti".to_string(),
        ))
    })?;

    driver.goto("https://linkedin.com").await?;
    driver.add_all_cookies(cookies.iter()).await?;

    info!("Sukurtas web driver");
    Ok(driver)
}

pub fn get_graduates_to_fetch() -> Result<Vec<String>, diesel::result::Error> {
    let conn = &mut establish_connection();

    imported_graduates::table
        .select(imported_graduates::id)
        .load::<String>(conn)
}
