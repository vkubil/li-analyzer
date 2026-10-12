use iced::widget::pane_grid::{self, DragEvent};
use log::info;

use crate::ui::{app::ApplicationState, state::Message};

pub fn update(state: &mut ApplicationState, message: Message) {
    match message {
        Message::Import => {
            info!("Import");
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
            info!("Message: {:?}: {:?}", split, ratio);
        }
        Message::PaneDragged(event) => match event {
            DragEvent::Dropped { pane, target } => {
                state.panes.drop(pane, target);
            }
            _ => {}
        },
    }
}
