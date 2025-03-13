use std::path::PathBuf;

use iced::highlighter::Theme;
use iced::widget::text_editor;
use iced::{Element, Length};

use crate::window::Message;

pub fn display_file<'a>(filename: &str, content: &'a text_editor::Content) -> Element<'a, Message> {
    let path_buf = PathBuf::from(filename.to_owned());
    let ext = path_buf
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("txt");
    text_editor(content)
        .height(Length::Fill)
        .wrapping(iced::widget::text::Wrapping::WordOrGlyph)
        .highlight(ext, Theme::SolarizedDark)
        .on_action(Message::TextEditorAction)
        .into()
}
