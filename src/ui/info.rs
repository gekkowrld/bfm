use std::path::PathBuf;

use crate::config::config::Config;
use crate::files::dir;
use crate::ui::file_container::box_display;
use iced::widget::{Column, row, scrollable};
use iced::widget::{rich_text, span};
use iced::{Element, Length};

pub fn directory_information(path: PathBuf) -> Element<'static, crate::window::files::Message> {
    let config = Config::new().get_column_width();
    let directory = dir::directory_content(path);
    // Name,type,size
    let mut column = Column::new().push(row![
        rich_text![span("Name").size(50)].width(Length::Fixed(config.name)),
        rich_text![span("Type").size(50)].width(Length::Fixed(config.type_)),
        rich_text![span("Size").size(50)].width(Length::Fixed(config.size)),
    ]);

    for file in directory.files {
        column = column.push(box_display(file));
    }

    scrollable(column).into()
}
