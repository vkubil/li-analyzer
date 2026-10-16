use iced::{
    Task,
    widget::pane_grid::{self, DragEvent},
};
use log::error;
use log::info;

use crate::{
    config::CONFIG,
    ui::{
        app::ApplicationState,
        functions,
        state::{Message, ToastType},
    },
    web::{auth::authenticate_in_driver, create::create_web_driver},
};

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
        Message::CreateWebDriver => Task::perform(
            async {
                let web_driver = create_web_driver().await;

                if let Err(e) = web_driver {
                    return Err(e.into());
                }

                let web_driver = web_driver.unwrap();

                authenticate_in_driver(&web_driver, &CONFIG.credentials)
                    .await
                    .map(|_| web_driver)
            },
            handle_errors!(driver: { Message::FetchData(driver) }),
        ),
        Message::FetchData(_) => {
            info!("Prisijungta su Selenium");
            Task::none()
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
