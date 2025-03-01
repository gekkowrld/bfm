use super::file_icon::Assets;

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
}

pub struct DisplayTheme {
    pub row_style: fn(&Theme) -> Style,
    pub row_style_hovered: fn(&Theme) -> Style,
}

impl DisplayTheme {
    pub fn new() -> Self {
        Self {
            row_style: Self::row_style,
            row_style_hovered: Self::row_style_selected,
        }
    }

    pub fn row_style(_: &Theme) -> Style {
        let theme_string = String::from_iter(Self::get_theme("dark").iter().map(|b| *b as char));
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
        let theme_string = String::from_iter(Self::get_theme("dark").iter().map(|b| *b as char));
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

    fn get_theme(key: &str) -> Vec<u8> {
        Assets::get(format!("themes/{key}.toml").as_str())
            .unwrap()
            .data
            .into_owned()
    }
}

pub fn hex_to_color(color: String) -> Color {
    let color_hex = color.replace('#', "0x");

    let color_u32 = u32::from_str_radix(&color_hex[2..], 16);

    color!(color_u32.unwrap())
}
