use diesel::{RunQueryDsl, query_dsl::methods::SelectDsl};
use thirtyfour::WebDriver;

use crate::{
    database::{connection::establish_connection, schema::imported_graduates},
    web::util::url::get_profile_url,
};

pub async fn fetch_all_graduates(driver: WebDriver) -> anyhow::Result<()> {
    let graduates_to_fetch = get_graduates_to_fetch()?;

    for graduate_id in graduates_to_fetch {
        fetch_single_graduate(&driver, &graduate_id).await?;
    }
    Ok(())
}

pub async fn fetch_single_graduate<'a>(
    driver: &'a WebDriver,
    graduate_id: &'a str,
) -> anyhow::Result<()> {
    driver.goto(get_profile_url(&graduate_id)).await?;
    let title = driver.title().await?;
    let student_name = title.split('|').next().unwrap();

    println!("Absolvento vardas: {:?}", student_name);
    Ok(())
}

fn get_graduates_to_fetch() -> Result<Vec<String>, diesel::result::Error> {
    let conn = &mut establish_connection();

    imported_graduates::table
        .select(imported_graduates::id)
        .load::<String>(conn)
}
