use iced::border::Radius;
use iced::widget::container;
use iced::{Border, Theme};

pub fn title_bar(theme: &Theme) -> container::Style {
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
pub fn error(theme: &Theme) -> container::Style {
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
