use iced::widget::pane_grid::{self};

#[derive(Debug, Clone)]
pub enum ToastType {
    Error,
    Success,
}

#[derive(Debug, Clone)]
pub enum Message {
    FetchData,
    Import,
    Toast(ToastType, String),
    AddGraduates(Vec<String>),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}
