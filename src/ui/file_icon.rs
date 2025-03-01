use crate::window::files::Message;
use iced::Color;
use iced::{Element, Length, widget::svg};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/*.svg"]
#[include = "themes/*.toml"]
pub struct Assets;

pub fn icon<'a>(is_dir: bool, color: Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(get_icon(if is_dir {
        "dir"
    } else {
        "file"
    })))
    .width(Length::Fill)
    .style(move |_, _| svg::Style { color: Some(color) })
    .into()
}

fn get_icon(key: &str) -> Vec<u8> {
    Assets::get(format!("icons/{key}.svg").as_str())
        .unwrap()
        .data
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_icon() {
        let icon = get_icon("dir");
        assert_eq!(icon.len(), 303);
    }
}
