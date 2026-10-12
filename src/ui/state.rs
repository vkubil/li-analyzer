use iced::widget::pane_grid::{self};

#[derive(Debug, Clone)]
pub enum Message {
    Import,
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}
