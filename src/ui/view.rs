use iced::{
    Element, Length,
    widget::{
        PaneGrid, Row, column, container,
        pane_grid::{self, TitleBar},
        row, text,
    },
};

use crate::ui::{
    app::ApplicationState,
    panes::{self, UiPane},
    state::Message,
};

pub fn view(state: &ApplicationState) -> Element<'_, Message> {
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
                        .style(panes::style::pane_error),
                )
                .padding(16)
                .into(),
            })
            .title_bar(title_bar)
            .style(panes::style::pane)
        })
        .spacing(8)
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized)
    ]
    .padding(8)
    .into()
}

pub fn create_pane_title(pane: &UiPane) -> Row<'_, Message> {
    row![text(pane.r#type.title()),].spacing(5)
}
pub fn create_pane_title_bar(title: Row<'_, Message>) -> TitleBar<'_, Message> {
    pane_grid::TitleBar::new(title)
        .padding(10)
        .style(panes::style::pane_title_bar)
}
