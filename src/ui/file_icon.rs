use crate::window::files::Message;
use iced::color;
use iced::{Element, Length, widget::svg};

pub fn icon<'a>(is_dir: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(get_icon(if is_dir {
        "directory".to_string()
    } else {
        "file".to_string()
    })))
    .width(Length::Fill)
    .style(|_, _| svg::Style {
        color: Some(color!(0x83a300)),
    })
    .into()
}

fn get_icon(key: String) -> &'static [u8] {
    match key.as_str() {
        "file" => include_bytes!("../../icons/file.svg"),
        "directory" => include_bytes!("../../icons/dir.svg"),
        _ => include_bytes!("../../icons/file.svg"),
    }
}
