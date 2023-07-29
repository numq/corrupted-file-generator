use iced::{Background, Color, Theme};
use iced::theme::Button;
use iced::widget::{button, container};

pub struct CustomTheme;

struct ButtonPickSizeType;

struct ButtonSave;

impl CustomTheme {
    pub fn button_pick_size_type() -> Button {
        Button::Custom(Box::from(ButtonPickSizeType))
    }
    pub fn button_save() -> Button {
        Button::Custom(Box::from(ButtonSave))
    }
}

impl container::StyleSheet for ButtonPickSizeType {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            ..Default::default()
        }
    }
}

impl button::StyleSheet for ButtonPickSizeType {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(5, 140, 66))),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: self.active(style).background,
            ..Default::default()
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: self.active(style).background,
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(22, 219, 101))),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for ButtonSave {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(22, 219, 101))),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: self.active(_style).background,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: self.active(_style).background,
            ..Default::default()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}