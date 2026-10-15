use iced::Element;
use iced::widget::text;

use crate::ui::state::Message;

pub fn view<'a>() -> Element<'a, Message> {
    text("Data page...").into()
}
