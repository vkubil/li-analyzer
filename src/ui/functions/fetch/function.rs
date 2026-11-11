use std::{iter, sync::Arc};

use futures::{
    future::{join_all, try_join_all},
    lock::Mutex,
};
use thirtyfour::{
    Cookie, WebDriver,
    error::{WebDriverError, WebDriverResult},
};

use crate::{
    database::models::Graduate,
    ui::functions::fetch::{
        structure::GraduateInfo,
        util::{create_driver_for_fetch, get_graduates_to_fetch},
    },
    web::util::url::get_profile_url,
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

async fn dispose_driver<'a>(driver: &'a WebDriver) -> WebDriverResult<()> {
    driver.clone().close_window().await?;
    driver.clone().quit().await
}

async fn fetch_graduate_task<'a>(
    driver: &'a WebDriver,
    graduates_volatile: &'a Arc<Mutex<Vec<String>>>,
) -> Result<Vec<GraduateInfo>, WebDriverError> {
    let mut graduates = Vec::new();

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

        graduates.push(fetch_single_graduate(&driver, &graduate_id.unwrap()).await?);
    }

    Ok(graduates)
}

async fn fetch_single_graduate<'a>(
    driver: &'a WebDriver,
    graduate_id: &'a str,
) -> Result<GraduateInfo, WebDriverError> {
    driver.goto(get_profile_url(&graduate_id)).await?;
    let title = driver.title().await?;
    let student_name = title.split('|').next().unwrap();

    let graduate = Graduate {
        id: graduate_id.to_string(),
        full_name: student_name.to_string(),
    };

    Ok(GraduateInfo {
        graduate,
        courses: vec![],
        positions: vec![],
    })
}
