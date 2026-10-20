use iced::Element;

use crate::ui::{pages, state::Message};

pub enum PaneType {
    Fetch,
    Data,
}

impl PaneType {
    pub fn title(&self) -> String {
        match self {
            PaneType::Data => "Duomenys".to_string(),
            PaneType::Fetch => "Absolventų Importavimas".to_string(),
        }
    }
    pub fn view<'a>(&self) -> impl Fn() -> Result<Element<'a, Message>, anyhow::Error> {
        match self {
            PaneType::Data => pages::data::view,
            PaneType::Fetch => pages::fetch::view,
        }
    }
}
impl ToString for PaneType {
    fn to_string(&self) -> String {
        match self {
            PaneType::Data => "Data".to_string(),
            PaneType::Fetch => "Fetch".to_string(),
        }
    }
}

pub struct UiPane {
    pub r#type: PaneType,
}
impl UiPane {
    pub fn new(r#type: PaneType) -> Self {
        UiPane { r#type }
    }
}
