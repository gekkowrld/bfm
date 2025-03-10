use iced::Element;
use iced::widget::container;
use iced::widget::text::LineHeight;
use iced::widget::{scrollable, text};

use crate::window::Message;

pub fn file(path: String) -> Element<'static, Message> {
    let content = vfs::read_file(vfs::FS::Local, path.as_str()).unwrap();
    file_display(content.content)
}

pub fn file_display(content: String) -> Element<'static, Message> {
    scrollable(container(text!("{}", content).line_height(LineHeight::Relative(1.2))).padding(5))
        .into()
}
