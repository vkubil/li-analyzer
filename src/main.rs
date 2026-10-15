use crate::{
    config::{CONFIG, create_logger},
    ui::{app::ApplicationState, subscription::subscription},
};
use iced::Theme;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_logger(&CONFIG.logging);

    // let caps = DesiredCapabilities::firefox();
    // let driver = WebDriver::managed(caps).await?;
    // authenticate_in_driver(&driver, &config.credentials)
    //     .await
    //     .inspect_err(|e| error!("Klaida prisijungiant: {}", e))?;

    // info!("Prisijungta");

    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).expect("");

    // driver.quit().await?;

    iced::application(
        ApplicationState::default,
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
