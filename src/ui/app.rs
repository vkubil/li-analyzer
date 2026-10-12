use iced::widget::pane_grid::{self};

use crate::ui::panes::UiPane;

pub struct ApplicationState {
    pub panes: pane_grid::State<UiPane>,
    pub focused_pane: Option<pane_grid::Pane>,
}

impl ApplicationState {
    pub fn new() -> Self {
        let (panes, _) = pane_grid::State::new(UiPane::new(0));

        ApplicationState {
            panes,
            focused_pane: None,
        }
    }
}
impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState::new()
    }
}
