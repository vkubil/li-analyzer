use diesel::RunQueryDsl;
use iced::alignment::Horizontal;
use iced::widget::{button, column, scrollable, table, text};
use iced::{Element, Length};

use crate::database::connection::establish_connection;
use crate::database::models::ImportedGraduate;
use crate::database::schema::imported_graduates;
use crate::ui::state::Message;

fn id_view<'a>(value: ImportedGraduate) -> Element<'a, Message> {
    text(value.clone().id).into()
}

pub fn view<'a>() -> Result<Element<'a, Message>, anyhow::Error> {
    let conn = &mut establish_connection();
    let graduates = imported_graduates::table.load::<ImportedGraduate>(conn)?;

    Ok(column![
        scrollable(
            table::table(
                vec![table::column(text("Absolventas"), id_view),],
                graduates
            )
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill),
        button("Importuoti absolventus").on_press(Message::Import),
    ]
    .padding(16)
    .align_x(Horizontal::Center)
    .spacing(2)
    .width(Length::Fill)
    .height(Length::Fill)
    .into())
}
