use diesel::result::Error;
use iced::Element;
use iced::widget::text;

use crate::ui::state::Message;

pub fn view<'a>() -> Result<Element<'a, Message>, anyhow::Error> {
    Ok(text("Data page...").into())
}
