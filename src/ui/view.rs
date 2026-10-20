use iced::{
    Element, Length,
    widget::{
        PaneGrid, Row, column, container,
        pane_grid::{self, TitleBar},
        row, text,
    },
};

use crate::ui::{app::ApplicationState, panes::UiPane, state::Message, style};

pub fn view<'a>(state: &'a ApplicationState) -> Element<'a, Message> {
    column![
        PaneGrid::new(&state.panes, |_, pane, _| {
            let title = create_pane_title(pane);
            let title_bar = create_pane_title_bar(title);

            let pane_view = pane.r#type.view();

            pane_grid::Content::new(match pane_view() {
                Ok(elem) => elem,
                Err(err) => container(
                    container(text(format!("{:?}", err)))
                        .padding(8)
                        .width(Length::Fill)
                        .style(style::pane::error),
                )
                .padding(16)
                .into(),
            })
            .title_bar(title_bar)
            .style(style::pane::pane)
        })
        .spacing(8)
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized)
    ]
    .padding(8)
    .into()
}

pub fn create_pane_title<'a>(pane: &UiPane) -> Row<'a, Message> {
    row![text(pane.r#type.title()),].spacing(5)
}
pub fn create_pane_title_bar<'a>(title: Row<'a, Message>) -> TitleBar<'a, Message> {
    pane_grid::TitleBar::new(title)
        .padding(10)
        .style(style::pane::title_bar)
}
