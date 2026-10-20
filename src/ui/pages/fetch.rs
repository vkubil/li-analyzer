use diesel::RunQueryDsl;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, row, scrollable, table, text};
use iced::{Element, Length};

use crate::database::connection::establish_connection;
use crate::database::models::ImportedGraduate;
use crate::database::schema::imported_graduates;
use crate::ui::state::Message;
use crate::ui::style;

fn id_view<'a>(value: ImportedGraduate) -> Element<'a, Message> {
    text(value.clone().id).into()
}

pub fn view<'a>() -> Result<Element<'a, Message>, anyhow::Error> {
    let conn = &mut establish_connection();
    let graduates = imported_graduates::table.load::<ImportedGraduate>(conn)?;

    Ok(column![
        row![
            button("Importuoti")
                .style(style::button::neutral::left)
                .on_press(Message::Import),
            button("Gauti informacija")
                .style(style::button::neutral::center)
                .on_press(Message::FetchData)
                .width(Length::Fill),
            button("Ištrinti importuotus")
                .style(style::button::danger::right)
                .on_press(Message::ClearImports),
        ]
        .align_y(Vertical::Center)
        .spacing(2)
        .width(Length::Fill),
        scrollable(
            table::table(
                vec![table::column(text("Absolventas"), id_view),],
                graduates
            )
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ]
    .padding(16)
    .align_x(Horizontal::Center)
    .spacing(2)
    .width(Length::Fill)
    .height(Length::Fill)
    .into())
}
