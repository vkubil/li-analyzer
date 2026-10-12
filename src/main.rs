use crate::{
    config::{create_logger, read_config},
    ui::{app::ApplicationState, subscription::subscription},
};
use iced::Theme;
use log::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = read_config();

    if let Err(e) = config {
        error!("Klaida nuskaitant konfiguraciją: {}", e);

        return Err(e);
    }

    let config = config.unwrap();

    create_logger(&config.logging);

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
pub mod ui;
pub mod web;
