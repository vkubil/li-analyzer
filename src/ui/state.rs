use iced::widget::pane_grid::{self};
use thirtyfour::WebDriver;

#[derive(Debug, Clone)]
pub enum ToastType {
    Error,
    Success,
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateWebDriver,
    FetchData(WebDriver),
    Import,
    Toast(ToastType, String),
    AddGraduates(Vec<String>),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}
