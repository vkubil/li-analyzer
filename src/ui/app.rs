use iced::widget::pane_grid::{self, Axis};

use crate::ui::panes::UiPane;

pub struct ApplicationState {
    pub panes: pane_grid::State<UiPane>,
}

impl ApplicationState {
    pub fn new() -> Self {
        let (mut panes, first_pane) =
            pane_grid::State::new(UiPane::new(super::panes::PaneType::Data));
        panes.split(
            Axis::Vertical,
            first_pane,
            UiPane::new(super::panes::PaneType::Fetch),
        );

        ApplicationState { panes }
    }
}
impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState::new()
    }
}
