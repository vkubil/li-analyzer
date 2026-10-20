use thirtyfour::{
    By, WebDriver, WebElement, error::WebDriverError, extensions::query::ElementQueryable,
};

const WORKSPACE_ID: &str = "workspace";

pub async fn get_details_page(driver: &WebDriver) -> Result<WebElement, WebDriverError> {
    driver
        .query(By::Id(WORKSPACE_ID))
        .desc("Pagrindinis detaliu puslapio elementas")
        .single()
        .await
}

// const WORKSPACE_ITEM_CSS: &str = "[componentkey^=\"entity-collection-item\"]";

pub fn get_profile_details(profile_url: &str, page: &str) -> String {
    format!("{}/details/{}/", profile_url, page)
}
