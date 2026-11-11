use iced::Element;

use crate::ui::{pages, state::Message};

pub enum PaneType {
    FetchGraduates,
    FetchCourses,
    Data,
}

impl PaneType {
    pub fn title(&self) -> String {
        match self {
            PaneType::Data => "Duomenys".to_string(),
            PaneType::FetchGraduates => "Absolventų Importavimas".to_string(),
            PaneType::FetchCourses => "Kursų Importavimas".to_string(),
        }
    }
    pub fn view<'a>(&self) -> impl Fn() -> Result<Element<'a, Message>, anyhow::Error> {
        match self {
            PaneType::Data => pages::data::view,
            PaneType::FetchGraduates => pages::fetch_graduates::view,
            PaneType::FetchCourses => pages::fetch_courses::view,
        }
    }
}
impl ToString for PaneType {
    fn to_string(&self) -> String {
        match self {
            PaneType::Data => "Data".to_string(),
            PaneType::FetchGraduates => "Fetch Graduates".to_string(),
            PaneType::FetchCourses => "Fetch Courses".to_string(),
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
