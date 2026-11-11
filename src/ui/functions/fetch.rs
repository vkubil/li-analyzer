use std::{iter, sync::Arc};

use diesel::{RunQueryDsl, query_dsl::methods::SelectDsl};
use futures::{
    future::{join_all, try_join_all},
    lock::Mutex,
};
use thirtyfour::{
    Cookie, WebDriver,
    error::{WebDriverError, WebDriverErrorInfo, WebDriverResult},
};

use crate::{
    database::{connection::establish_connection, schema::imported_graduates},
    web::{
        create::create_web_driver, extensions::WebDriverCookieExtension, util::url::get_profile_url,
    },
};
use log::{info, warn};

pub async fn fetch_all_graduates(
    driver_cookies: Vec<Cookie>,
    chunk_length: usize,
    drivers: usize,
) -> anyhow::Result<()> {
    let graduates_to_fetch = get_graduates_to_fetch()?;

    let graduates_chunked = graduates_to_fetch.chunks(chunk_length);
    // Nes bus visi kiti browser sessions be cookies
    let cookies = Arc::new(driver_cookies);

    let drivers_for_use = try_join_all(
        iter::repeat_with(|| async { create_driver_for_fetch(&cookies).await }).take(drivers),
    )
    .await?;

    info!("Gaunama informacija");
    for graduate_chunk in graduates_chunked {
        let graduates_volatile = Arc::new(Mutex::new(Vec::from(graduate_chunk)));

        // Atlikti viska kartu, o ne vienas po kito
        let fetched_all = join_all(
            drivers_for_use
                .iter()
                .map(|driver| fetch_graduate_task(&driver, &graduates_volatile)),
        )
        .await;

        // Vietoj to, kad crashintu viskas, geriau, kad informuotu apie kiekviena klaida atskirai ir naudotis gerais veiksmais
        for fetched in fetched_all {
            match fetched {
                Ok(values) => info!("Gautas rezultatas: {:?}", values),
                Err(e) => warn!("Klaida, kai bandyta gauti rezultata: {:?}", e),
            }
        }
    }

    try_join_all(drivers_for_use.iter().map(dispose_driver)).await?;

    Ok(())
}

pub async fn dispose_driver<'a>(driver: &'a WebDriver) -> WebDriverResult<()> {
    driver.clone().close_window().await?;
    driver.clone().quit().await
}

pub async fn create_driver_for_fetch(cookies: &Arc<Vec<Cookie>>) -> WebDriverResult<WebDriver> {
    let driver = create_web_driver().await?;

    driver.clone().leak().map_err(|_| {
        WebDriverError::UnknownError(WebDriverErrorInfo::new(
            "Klaida leak'inant driverį užduočiai atlikti".to_string(),
        ))
    })?;
    info!("Pridedami cookies");
    driver.goto("https://linkedin.com").await?;
    driver.add_all_cookies(cookies.iter()).await?;
    info!("Prideti cookies");

    Ok(driver)
}

pub async fn fetch_graduate_task<'a>(
    driver: &'a WebDriver,
    graduates_volatile: &'a Arc<Mutex<Vec<String>>>,
) -> Result<Vec<String>, WebDriverError> {
    let mut titles = Vec::new();

    loop {
        // Gauti graduate_id sitam tabui
        let mut lock = graduates_volatile.lock().await;
        let graduate_id = lock.pop();
        // Atrakinti mutex, kad kiti tabs galetu naudotis juo
        drop(lock);

        // Jei nera graduate it, tai uzbaikti visa tai
        if graduate_id.is_none() {
            break;
        }

        titles.push(fetch_single_graduate(&driver, &graduate_id.unwrap()).await?);
    }

    Ok(titles)
}

pub async fn fetch_single_graduate<'a>(
    driver: &'a WebDriver,
    graduate_id: &'a str,
) -> Result<String, WebDriverError> {
    driver.goto(get_profile_url(&graduate_id)).await?;
    let title = driver.title().await?;
    let student_name = title.split('|').next().unwrap();

    println!("Absolvento vardas: {:?}", student_name);
    Ok(student_name.to_string())
}

fn get_graduates_to_fetch() -> Result<Vec<String>, diesel::result::Error> {
    let conn = &mut establish_connection();

    imported_graduates::table
        .select(imported_graduates::id)
        .load::<String>(conn)
}
