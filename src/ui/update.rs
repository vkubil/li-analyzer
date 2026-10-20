use iced::{
    Task,
    widget::pane_grid::{self, DragEvent},
};
use log::error;
use log::info;

use crate::ui::{
    app::ApplicationState,
    functions::{self, remove_imports::remove_all_imported},
    state::{Message, ToastType},
};

macro_rules! handle_sync_errors {
    ($expression: expr; $name: ident: $success: expr) => {
        match $expression {
            Ok($name) => $success,
            Err(err) => {
                return Task::done(Message::Toast(ToastType::Error, err.to_string()));
            }
        }
    };
}
macro_rules! handle_sync_info {
    ($expression: expr; $success: expr) => {
        handle_sync_errors!($expression; _res: { return Task::done(Message::Toast(ToastType::Success, $success.to_string())); });
    };
}
macro_rules! handle_errors {
    ($name: ident: $expression: expr) => {
        |result| match result {
            Ok($name) => $expression,
            Err(err) => Message::Toast(ToastType::Error, err.to_string()),
        }
    };
}

pub fn update(state: &mut ApplicationState, message: Message) -> Task<Message> {
    match message {
        Message::FetchData => {
            info!("Prisijungta su Selenium");
            Task::none()
        }
        Message::ClearImports => {
            handle_sync_info!(remove_all_imported(); "Ištrinti importuoti absolventai");
        }
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
                handle_errors!(value: Message::AddGraduates(value)),
            )
        }
        Message::AddGraduates(ids) => {
            info!("Importuoti IDs: {:?}", ids);
            match functions::add_graduates::add_graduates(ids) {
                Ok(count) => info!("Importuoti {} absolventai", count),
                Err(err) => error!("{:?}", err),
            };
            Task::none()
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);

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
