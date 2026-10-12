use iced::{
    Subscription,
    keyboard::{self, Key},
};
use log::info;

use crate::ui::{app::ApplicationState, state::Message};

pub fn subscription(state: &ApplicationState) -> Subscription<Message> {
    keyboard::listen().filter_map(|event| {
        let keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
            return None;
        };

        if !modifiers.command() {
            return None;
        }

        handle_keyboard(key)
    })
}

pub fn handle_keyboard(key: keyboard::Key) -> Option<Message> {
    match key.as_ref() {
        Key::Character("v") => {
            info!("V key press");
            None
        }
        _ => None,
    }
}
