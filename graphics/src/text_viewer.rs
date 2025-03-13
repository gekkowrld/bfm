use iced::Element;
use iced::widget::container;
use iced::widget::text::LineHeight;
use iced::widget::{scrollable, text};
use std::io::ErrorKind;
use std::io::Result;

use crate::window::Message;

pub fn file(path: String) -> Element<'static, Message> {
    let content = vfs::read_file(vfs::FS::Local, path.as_str());
    file_display(&content)
}

pub fn file_display(content: &Result<vfs::FileInformation>) -> Element<'static, Message> {
    let content = match content {
        Ok(content) => String::from_utf8_lossy(&content.content).to_string(),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => "File not found".to_string(),
            _ => "Failed to read file".to_string(),
        },
    };

    scrollable(container(text!("{}", content).line_height(LineHeight::Relative(1.2))).padding(5))
        .into()
}
