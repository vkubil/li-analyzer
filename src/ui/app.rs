use iced::widget::pane_grid::{self, Axis};
use thirtyfour::Cookie;
use tokio::runtime::Handle;

use crate::ui::panes::UiPane;

pub struct ApplicationState {
    pub panes: pane_grid::State<UiPane>,
    pub cookies: Vec<Cookie>,
    pub runtime: Handle,
}

impl ApplicationState {
    pub fn new(cookies: &Vec<Cookie>, runtime: Handle) -> Self {
        let (mut panes, first_pane) =
            pane_grid::State::new(UiPane::new(super::panes::PaneType::Data));

        panes.split(
            Axis::Vertical,
            first_pane,
            UiPane::new(super::panes::PaneType::Fetch),
        );

        ApplicationState {
            panes,
            cookies: cookies.clone(),
            runtime,
        }
    }
}
