use chrono::{NaiveDate, ParseResult};
use thirtyfour::{
    By, WebDriver, WebElement, error::WebDriverError, extensions::query::ElementQueryable,
};

const WORKSPACE_ID: &str = "workspace";

pub async fn get_details_page(driver: &WebDriver) -> Result<WebElement, WebDriverError> {
    driver
        .query(By::Id(WORKSPACE_ID))
        .desc("Pagrindinis detaliu (patirties) puslapio elementas")
        .single()
        .await
}

const WORKSPACE_ITEM_CSS: &str = "[componentkey^=\"entity-collection-item\"]";

pub async fn get_detail_items(workspace: &WebElement) -> Result<Vec<WebElement>, WebDriverError> {
    workspace.query(By::Css(WORKSPACE_ITEM_CSS)).any().await
}

pub struct DateRange {
    pub from: NaiveDate,
    pub to: Option<NaiveDate>,
}

impl DateRange {
    pub fn from_text(range: &str) -> ParseResult<DateRange> {
        let first_part = range
            .split('·')
            .next()
            .expect(format!("Negalėjo gauti data iš {:?}", range).as_str());

        let split = first_part.split('-').collect::<Vec<&str>>();
        let from_str = split.first().unwrap().trim();
        let to_str = split.last().unwrap().trim();

        Ok(DateRange {
            from: parse_from_only_month_and_year(from_str)?,
            to: match to_str {
                "Present" => None,
                to_str => parse_from_only_month_and_year(to_str).ok(),
            },
        })
    }
}

fn parse_from_only_month_and_year(value: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(format!("{} 01", value).as_str(), "%b %Y %d")
}
