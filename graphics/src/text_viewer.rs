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
        Ok(file) => file.content.clone(),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => format!("File not found:\n {:?}", err.to_string()),
            ErrorKind::InvalidData => {
                format!(
                    "NOT A TEXT FILE, BINARY FILES ARE NOT SUPPORTED:\n {:?}",
                    err.to_string()
                )
            }
            _ => format!("Error: {:?}", err),
        },
    };

    scrollable(container(text!("{}", content).line_height(LineHeight::Relative(1.2))).padding(5))
        .into()
}
