use crate::fs::embed::Assets;

use iced::{Background, Theme};
use mlua::{Lua, Table};

pub enum Component {
    Box,
}

pub fn component_style(component: Component) -> impl Fn(Theme) -> iced::widget::container::Style {
    move |_theme| match component {
        Component::Box => iced::widget::container::Style {
            background: Some(Background::Color(iced::Color::from_rgb8(0x1A, 0x1A, 0x1A))),
            text_color: Some(iced::Color::WHITE),
            ..iced::widget::container::Style::default()
        },
    }
}

pub struct LuaValues {
    pub window_background: iced::Color,
    pub text_color: iced::Color,
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

    let window_background: mlua::Table = theme.get("window_background").unwrap();
    let text_color: mlua::Table = theme.get("text_color").unwrap();

    LuaValues {
        window_background: lua_color_to_iced_color(window_background),
        text_color: lua_color_to_iced_color(text_color),
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
