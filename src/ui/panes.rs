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
            PaneType::Fetch => "Eksportavimas ir Importavimas".to_string(),
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

pub mod style {
    use iced::border::Radius;
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn pane_title_bar(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.weakest.text),
            background: Some(palette.background.weakest.color.into()),
            border: Border {
                color: palette.background.weaker.color,
                width: 1.0,
                radius: Radius::new(0).top(8),
            },
            ..Default::default()
        }
    }
    pub fn pane(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weakest.color.into()),
            border: Border {
                color: palette.background.weaker.color,
                width: 1.0,
                radius: Radius::new(8),
            },
            ..Default::default()
        }
    }
    pub fn pane_error(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.danger.weak.color.into()),
            text_color: Some(palette.danger.weak.text.into()),
            border: Border {
                radius: Radius::new(4),
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
