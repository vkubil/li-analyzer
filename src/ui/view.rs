use iced::{
    Element,
    widget::{button, column, pane_grid, responsive, text},
};

use crate::ui::{
    app::ApplicationState,
    panes::{PaneState, UiPane},
    state::Message,
};

pub fn view(state: &ApplicationState) -> Element<'_, Message> {
    pane_grid(&state.panes, |pane, state, fullscreen| {
        pane_grid::Content::new(text("Example pane"))
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .into()
    // column![
    //     text("Absolventų sąrašas").size(20),
    //     button("Importuoti absolventų sąrašą").on_press(Message::Import)
    // ]
    // .spacing(10)
    // .into()
}
