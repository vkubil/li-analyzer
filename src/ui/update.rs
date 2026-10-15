use iced::{
    Task,
    widget::pane_grid::{self, DragEvent},
};
use log::error;
use log::info;

use crate::ui::{
    app::ApplicationState,
    functions,
    state::{Message, ToastType},
};

pub fn update(state: &mut ApplicationState, message: Message) -> Task<Message> {
    match message {
        Message::Toast(r#type, message) => {
            match r#type {
                ToastType::Error => {
                    error!("{}", message);
                }
                ToastType::Success => {
                    info!("{}", message);
                }
            };
            Task::none()
        }
        Message::Import => {
            info!("Importuojama");
            Task::perform(
                functions::import::import_graduate_files(),
                |result| match result {
                    Ok(value) => Message::Imported(value),
                    Err(err) => Message::Toast(ToastType::Error, err.to_string()),
                },
            )
        }
        Message::Imported(result) => {
            info!("Importuoti asmenys: {:?}", result);
            Task::none()
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
            info!("Message: {:?}: {:?}", split, ratio);
            Task::none()
        }
        Message::PaneDragged(event) => match event {
            DragEvent::Dropped { pane, target } => {
                state.panes.drop(pane, target);
                Task::none()
            }
            _ => Task::none(),
        },
    }
}
