use iced::widget::{button, container, text_input};
use iced::{Border, Color, Theme};

pub fn rounded_container(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            radius: 5.into(),
            color: palette.background.strong.color,
        },
        ..container::Style::default()
    }
}

pub fn instance_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let style = button::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 1.0,
            radius: 5.into(),
            color: palette.background.strong.color,
        },
        ..button::secondary(theme, status)
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.background.weak.color.scale_alpha(0.5).into()),
            ..style
        },
        _ => style,
    }
}

pub fn model_backdrop(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(
            Color {
                a: 0.8,
                ..Color::BLACK
            }
            .into(),
        ),
        ..container::Style::default()
    }
}

pub fn text_input_warning(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let default = text_input::default(theme, status);

    text_input::Style {
        border: Border {
            color: Color::from_rgb8(255, 0, 0),
            ..default.border
        },
        ..default
    }
}

pub fn striped(index: usize) -> impl Fn(&Theme) -> container::Style {
    if index % 2 == 0 {
        container::transparent
    } else {
        move |theme: &Theme| {
            let palette = theme.extended_palette();
            if palette.is_dark {
                container::background(palette.background.weak.color.scale_alpha(0.2))
            } else {
                container::background(palette.background.weak.color.scale_alpha(0.6))
            }
        }
    }
}
