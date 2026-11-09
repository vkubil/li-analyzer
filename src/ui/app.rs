use iced::widget::pane_grid::{self, Axis};
use thirtyfour::WebDriver;
use tokio::runtime::Handle;

use crate::ui::panes::UiPane;

pub struct ApplicationState {
    pub panes: pane_grid::State<UiPane>,
    pub driver: WebDriver,
    /// The runtime that owns the [`WebDriver`] connection pool.
    ///
    /// `thirtyfour` needs a Tokio reactor, so every task that talks to the
    /// driver must be spawned onto this handle instead of being polled
    /// directly by iced's executor.
    pub runtime: Handle,
}

impl ApplicationState {
    pub fn new(driver: &WebDriver, runtime: Handle) -> Self {
        let (mut panes, first_pane) =
            pane_grid::State::new(UiPane::new(super::panes::PaneType::Data));
        panes.split(
            Axis::Vertical,
            first_pane,
            UiPane::new(super::panes::PaneType::Fetch),
        );

        ApplicationState {
            panes,
            driver: driver.clone(),
            runtime,
        }
    }
}
