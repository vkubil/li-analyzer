use crate::{
    config::{CONFIG, create_logger},
    ui::{app::ApplicationState, subscription::subscription},
    web::{auth::authenticate_in_driver, create::create_web_driver},
};
use iced::Theme;
use log::{error, info, warn};
use thirtyfour::error::WebDriverErrorInner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_logger(&CONFIG.logging);
    let web_driver = create_web_driver().await;

    if let Err(e) = web_driver {
        return Err(e.into());
    }

    let web_driver = web_driver.unwrap();

    authenticate_in_driver(&web_driver, &CONFIG.credentials).await?;

    match web_driver.clone().quit().await {
        Ok(_) => {
            info!("Prisijungta");
        }
        Err(e) => match e.into_inner() {
            WebDriverErrorInner::Timeout(_) => {
                warn!("Galimai neprisijungė");
            }
            e => {
                error!("Klaida prisjungiant: {}", e);
                Err(e)?;
            }
        },
    }

    iced::application(
        move || ApplicationState::new(&web_driver),
        crate::ui::update::update,
        crate::ui::view::view,
    )
    .theme(Theme::CatppuccinMocha)
    .subscription(subscription)
    .run()
    .map_err(|err| err.into())
}

pub mod config;
pub mod database;
pub mod ui;
pub mod web;
