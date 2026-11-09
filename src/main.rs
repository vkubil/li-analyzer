use crate::{
    config::{CONFIG, create_logger},
    ui::{app::ApplicationState, subscription::subscription},
    web::{auth::authenticate_in_driver, create::create_web_driver},
};
use iced::Theme;
use log::info;

pub fn main() -> anyhow::Result<()> {
    create_logger(&CONFIG.logging);

    // Naudojant WebDriver funkcijas, gaunamas si klaida:
    // Cannot start a runtime from within a runtime. This happens because a function (like `block_on`)
    // attempted to block the current thread while the thread is being used to drive asynchronous tasks.
    // Taigi, reikia naudoti runtime atskirta nuo iced runtime, kad galetume normaliai naudotis Selenium
    let runtime = tokio::runtime::Runtime::new()?;
    let handle = runtime.handle().clone();

    // Sukurti web driver tame runtime
    let web_driver = handle.block_on(async {
        let driver = create_web_driver().await?;
        authenticate_in_driver(&driver, &CONFIG.credentials).await?;
        Ok::<_, anyhow::Error>(driver)
    })?;

    info!("Prisijungta");

    let ui = iced::application(
        move || ApplicationState::new(&web_driver, handle.clone()),
        crate::ui::update::update,
        crate::ui::view::view,
    )
    .theme(Theme::Dark)
    .subscription(subscription)
    .run();

    // Kai baigesi UI session, reikia sustabdyti runtime
    drop(runtime);

    ui.map_err(|err| err.into())
}

pub mod config;
pub mod database;
pub mod ui;
pub mod web;
