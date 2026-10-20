use iced::widget::pane_grid::{self, Axis};
use thirtyfour::WebDriver;

use crate::ui::panes::UiPane;

pub struct ApplicationState {
    pub panes: pane_grid::State<UiPane>,
    pub driver: WebDriver,
}

impl ApplicationState {
    pub fn new(driver: &WebDriver) -> Self {
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
        }
    }
}
