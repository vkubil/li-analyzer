use crate::{
    config::{CONFIG, create_logger},
    ui::{app::ApplicationState, subscription::subscription},
};
use iced::Theme;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_logger(&CONFIG.logging);

    iced::application(
        || ApplicationState::new(),
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
