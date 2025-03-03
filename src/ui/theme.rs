use crate::fs::embed::Assets;

use iced::{
    Background, Border, Color, Shadow, Theme, border::Radius, color, widget::container::Style,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct T {
    pub name: String,
    pub description: String,
    pub version: String,
    colors: Colors,
}

#[derive(Deserialize, Serialize)]
pub struct Colors {
    pub file_column: FileColumn,
    pub window: Window,
}

#[derive(Deserialize, Serialize)]
pub struct FileColumn {
    pub background: String,
    pub foreground: String,
    pub background_selected: String,
    pub foreground_selected: String,
    pub border: String,
    pub border_selected: String,
    pub border_width: f32,
    pub border_width_selected: f32,
    pub border_radius: f32,
    pub border_radius_selected: f32,
    pub icon_color: String,
    pub icon_color_selected: String,
}

#[derive(Deserialize, Serialize)]
pub struct Window {
    pub background: String,
}

pub struct DisplayTheme {
    pub row_style: fn(&Theme) -> Style,
    pub row_style_hovered: fn(&Theme) -> Style,
    pub icon_color: Color,
    pub icon_color_selected: Color,
    pub window_decoration: fn(&Theme) -> Style,
}

impl Default for DisplayTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayTheme {
    pub fn new() -> Self {
        Self {
            row_style: Self::row_style,
            row_style_hovered: Self::row_style_selected,
            icon_color: Self::icon_color(),
            icon_color_selected: Self::icon_color_selected(),
            window_decoration: Self::window_background,
        }
    }

    pub fn window_background(_: &Theme) -> Style {
        let theme_string = String::from_iter(Assets::get_theme("dark").iter().map(|b| *b as char));
        let config: T = toml::from_str(&theme_string).unwrap();
        let fc = config.colors.window;
        let ffc = config.colors.file_column;

        Style {
            text_color: Some(hex_to_color(ffc.foreground)),
            background: Some(Background::Color(hex_to_color(fc.background))),
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }

    pub fn row_style(_: &Theme) -> Style {
        let theme_string = String::from_iter(Assets::get_theme("dark").iter().map(|b| *b as char));
        let config: T = toml::from_str(&theme_string).unwrap();
        let fc = config.colors.file_column;

        Style {
            text_color: Some(hex_to_color(fc.foreground)),
            background: Some(Background::Color(hex_to_color(fc.background))),
            border: Border {
                color: hex_to_color(fc.border),
                width: fc.border_width,
                radius: Radius::new(fc.border_radius),
            },
            shadow: Shadow::default(),
        }
    }

    pub fn row_style_selected(_theme: &Theme) -> Style {
        let theme_string = String::from_iter(Assets::get_theme("dark").iter().map(|b| *b as char));
        let config: T = toml::from_str(&theme_string).unwrap();
        let fc = config.colors.file_column;

        Style {
            text_color: Some(hex_to_color(fc.foreground_selected)),
            background: Some(Background::Color(hex_to_color(fc.background_selected))),
            border: Border {
                color: hex_to_color(fc.border_selected),
                width: fc.border_width_selected,
                radius: Radius::new(fc.border_radius_selected),
            },
            shadow: Shadow::default(),
        }
    }

    pub fn icon_color() -> Color {
        let theme_string = String::from_iter(Assets::get_theme("dark").iter().map(|b| *b as char));
        let config: T = toml::from_str(&theme_string).unwrap();
        let fc = config.colors.file_column;

        hex_to_color(fc.icon_color)
    }

    pub fn icon_color_selected() -> Color {
        let theme_string = String::from_iter(Assets::get_theme("dark").iter().map(|b| *b as char));
        let config: T = toml::from_str(&theme_string).unwrap();
        let fc = config.colors.file_column;

        hex_to_color(fc.icon_color_selected)
    }
}

pub fn hex_to_color(color: String) -> Color {
    let color_hex = color.replace('#', "0x");

    let color_u32 = u32::from_str_radix(&color_hex[2..], 16);

    color!(color_u32.unwrap())
}
