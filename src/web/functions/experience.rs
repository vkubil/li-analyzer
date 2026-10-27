use thirtyfour::{By, WebElement, error::WebDriverError, extensions::query::ElementQueryable};

const EXPERIENCE_TIMELINE_ITEMS: &str = "ul li div:nth-of-type(2)";

pub async fn get_timeline_items(
    experience: &WebElement,
) -> Result<Vec<WebElement>, WebDriverError> {
    experience
        .query(By::Css(EXPERIENCE_TIMELINE_ITEMS))
        .desc("kelios patirtys vienoje kompanijoje")
        .any()
        .await
}

pub async fn get_headline_paragraphs_from(
    element: &WebElement,
) -> Result<Vec<WebElement>, WebDriverError> {
    element
        .query(By::Tag("a"))
        .with_attribute("tabindex", "0")
        .desc("patirties antraštė")
        .first()
        .await?
        .query(By::Tag("p"))
        .desc("patirties antraštės paragrafai")
        .any()
        .await
}
