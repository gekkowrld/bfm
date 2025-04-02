use crate::window::Message;
use iced::Element;
use iced::widget::{scrollable, text};
use vfs::DirectoryInformation as DI;

pub fn directory_info(dir: &DI) -> Element<'static, Message> {
    let files = &dir.files;
    let mut elements = vec![];
    for file in files {
        elements.push(crate::file_card::card(file));
    }
    scrollable(iced::widget::Column::with_children(elements).spacing(10)).into()
}

pub fn directory(path: &str) -> Element<Message> {
    let dir = match vfs::list_files(vfs::FS::Local, path) {
        Ok(dir) => dir,
        Err(err) => return text!("{path}:: {}", err.to_string()).into(),
    };
    directory_info(&dir)
}
