use crate::window::Message;
use iced::Element;
use iced::widget::scrollable;
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
    let dir = vfs::list_files(vfs::FS::Local, path).unwrap();
    directory_info(&dir)
}
