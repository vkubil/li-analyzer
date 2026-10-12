use iced::{
    Element,
    widget::{
        PaneGrid, Row, column,
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

            pane_grid::Content::new(text("Example pane"))
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
