use std::path::PathBuf;

use crate::config::conf::Config;
use crate::fs::file;
use crate::ui::file_container::box_display;
use iced::widget::{Column, row, scrollable};
use iced::widget::{rich_text, span};
use iced::{Element, Length};

pub fn directory_information(
    hover_id: String,
    path: PathBuf,
) -> Element<'static, crate::window::files::Message> {
    let config = Config::new().get_column_width();
    let directory = file::directory_content(path);

    let mut column = Column::new().push(row![
        rich_text![span("Name").size(50)].width(Length::Fixed(config.name)),
        rich_text![span("Type").size(50)].width(Length::Fixed(config.type_)),
        rich_text![span("Size").size(50)].width(Length::Fixed(config.size)),
    ]);

    for file in directory.files {
        if file.id == hover_id {
            column = column.push(box_display(iced::widget::container::dark, file));
        } else {
            column = column.push(box_display(iced::widget::container::bordered_box, file));
        }
    }

    scrollable(column.spacing(12)).into()
}
