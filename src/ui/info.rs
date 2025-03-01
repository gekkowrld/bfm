use crate::config::conf::Config;
use crate::fs::file::Directory;
use crate::ui::file_container::box_display;
use iced::widget::{Column, column, row, scrollable};
use iced::widget::{rich_text, span};
use iced::{Element, Length};

pub fn directory_information(
    directory: &Directory,
) -> Element<'static, crate::window::files::Message> {
    let config = Config::new().get_column_width();

    let header = Column::new().push(row![
        rich_text![span("Name").size(50)].width(Length::Fixed(config.name)),
        rich_text![span("Type").size(50)].width(Length::Fixed(config.type_)),
        rich_text![span("Size").size(50)].width(Length::Fixed(config.size)),
    ]);

    let children = column(
        directory
            .files
            .iter()
            .map(|file| box_display(iced::widget::container::bordered_box, file)),
    );

    scrollable(column![header, children].spacing(12)).into()
}
