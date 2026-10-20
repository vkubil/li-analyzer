macro_rules! declare_style {
    ($name: ident, $color: ident; active: $level_active: ident, hover: $level_hover: ident, pressed: $level_pressed: ident, disabled: $level_disabled: ident) => {
        pub mod $name {
            use iced::{
                Border, Color, Theme,
                border::Radius,
                widget::button::{self, Status},
            };

            fn default_config(theme: &Theme, status: Status) -> button::Style {
                let palette = theme.extended_palette();

                let (text_color, background): (Color, Color) = match status {
                    Status::Active => (
                        palette.$color.$level_active.text.into(),
                        palette.$color.$level_active.color.into(),
                    ),
                    Status::Hovered => (
                        palette.$color.$level_hover.text.into(),
                        palette.$color.$level_hover.color.into(),
                    ),
                    Status::Pressed => (
                        palette.$color.$level_pressed.text.into(),
                        palette.$color.$level_pressed.color.into(),
                    ),
                    Status::Disabled => (
                        palette.$color.$level_disabled.text.into(),
                        palette.$color.$level_disabled.color.into(),
                    ),
                };

                button::Style {
                    background: Some(background.into()),
                    text_color,
                    ..Default::default()
                }
            }

            pub fn default(theme: &Theme, status: Status) -> button::Style {
                button::Style {
                    border: Border {
                        radius: Radius::new(16),
                        ..Border::default()
                    },
                    ..default_config(theme, status)
                }
            }
            pub fn left(theme: &Theme, status: Status) -> button::Style {
                button::Style {
                    border: Border {
                        radius: Radius::new(16).right(4),
                        ..Border::default()
                    },
                    ..default_config(theme, status)
                }
            }
            pub fn center(theme: &Theme, status: Status) -> button::Style {
                button::Style {
                    border: Border {
                        radius: Radius::new(4),
                        ..Border::default()
                    },
                    ..default_config(theme, status)
                }
            }
            pub fn right(theme: &Theme, status: Status) -> button::Style {
                button::Style {
                    border: Border {
                        radius: Radius::new(16).left(4),
                        ..Border::default()
                    },
                    ..default_config(theme, status)
                }
            }
        }
    };
}

declare_style!(primary, primary; active: base, hover: strong, pressed: weak, disabled: weak);
declare_style!(danger, danger; active: base, hover: strong, pressed: weak, disabled: weak);
declare_style!(neutral, background; active: weak, hover: strong, pressed: weaker, disabled: weaker);
