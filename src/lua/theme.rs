use crate::fs::embed::Assets;

use iced::{Background, Theme, border::Radius};
use mlua::{Lua, Table};

pub enum Component {
    Box,
    BoxHovered,
}

pub fn component_style(component: Component) -> fn(&Theme) -> iced::widget::container::Style {
    match component {
        Component::Box | Component::BoxHovered => {
            fn c(_: &Theme) -> iced::widget::container::Style {
                let theme_v = get_theme("dark");
                iced::widget::container::Style {
                    background: Some(Background::Color(theme_v.background)),
                    text_color: Some(theme_v.color),
                    border: theme_v.border,
                    ..iced::widget::container::Style::default()
                }
            }

            c
        }
    }
}

pub struct LuaValues {
    pub theme_name: String,
    pub background: iced::Color,
    pub color: iced::Color,
    pub primary: iced::Color,
    pub success: iced::Color,
    pub warning: iced::Color,
    pub error: iced::Color,
    pub border: iced::Border,
}

pub fn get_theme(name: &str) -> LuaValues {
    let _dark_file = Assets::get_lua(name);
    let _dark_file = String::from_utf8(_dark_file).unwrap();
    let _theme_file = Assets::get_lua("_theme");
    let _theme_file = String::from_utf8(_theme_file).unwrap();

    let lua_code = Lua::new();

    lua_code.load(&_theme_file).exec().unwrap();
    lua_code.load(&_dark_file).exec().unwrap();

    let generated_theme: mlua::Table = lua_code.globals().get("GeneratedTheme").unwrap();

    let theme: mlua::Table = generated_theme.get("theme").unwrap();
    let metadata: mlua::Table = generated_theme.get("metadata").unwrap();

    let window_background: mlua::Table = theme.get("background").unwrap();
    let text_color: mlua::Table = theme.get("color").unwrap();
    let primary: mlua::Table = theme.get("primary_color").unwrap();
    let success: mlua::Table = theme.get("success_color").unwrap();
    let warning: mlua::Table = theme.get("warning_color").unwrap();
    let error: mlua::Table = theme.get("error_color").unwrap();
    let border: mlua::Table = theme.get("border").unwrap();

    LuaValues {
        theme_name: metadata.get("name").unwrap(),
        background: lua_color_to_iced_color(window_background),
        color: lua_color_to_iced_color(text_color),
        primary: lua_color_to_iced_color(primary),
        success: lua_color_to_iced_color(success),
        warning: lua_color_to_iced_color(warning),
        error: lua_color_to_iced_color(error),
        border: lua_to_border(border),
    }
}

fn lua_to_border(lua_border: Table) -> iced::Border {
    iced::Border {
        color: lua_color_to_iced_color(lua_border.get("color").unwrap()),
        width: lua_border.get("width").unwrap(),
        radius: lua_to_radius(lua_border.get("radius").unwrap()),
    }
}

fn lua_to_radius(lua_radius: Table) -> Radius {
    Radius {
        top_right: lua_radius.get("top_right").unwrap(),
        top_left: lua_radius.get("top_left").unwrap(),
        bottom_right: lua_radius.get("bottom_right").unwrap(),
        bottom_left: lua_radius.get("bottom_left").unwrap(),
    }
}

fn lua_color_to_iced_color(lua_color: Table) -> iced::Color {
    iced::Color::from_rgba8(
        lua_color.get("red").unwrap(),
        lua_color.get("green").unwrap(),
        lua_color.get("blue").unwrap(),
        lua_color.get("alpha").unwrap(),
    )
}
