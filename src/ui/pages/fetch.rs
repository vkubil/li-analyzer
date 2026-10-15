use iced::Element;
use iced::widget::{button, stack};

use crate::ui::state::Message;

pub fn view<'a>() -> Element<'a, Message> {
    stack![button("Importuoti absolventus").on_press(Message::Import)].into()
}
