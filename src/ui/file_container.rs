use iced::Length;
use iced::Theme;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::rich_text;
use iced::widget::row;
use iced::widget::span;

use crate::config::conf::Config;
use crate::files::file::File;
use crate::ui::file_icon::icon;
use crate::window::files::Message;
use std::path::Path;

pub fn box_display<'a>(
    box_style: fn(&Theme) -> iced::widget::container::Style,
    file_info: File,
) -> iced::Element<'a, Message> {
    let url = file_info.url.clone();
    let config = Config::new().get_column_width();
    let parent_url =
        url::Url::from_directory_path(Path::new(&url.path()).parent().unwrap()).unwrap();
    mouse_area(
        container(
            row![
                icon(file_info.is_dir),
                rich_text![span(
                    Path::new(&url.path().to_owned())
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                )]
                .width(Length::Fixed(config.name)),
                rich_text![span(file_type(&file_info))].width(Length::Fixed(config.type_)),
                rich_text![span(file_info.file.metadata().unwrap().len().to_string())]
                    .width(Length::Fixed(config.type_)),
            ]
            .height(Length::Fixed(42.2))
            .width(Length::Fill),
        )
        .style(box_style),
    )
    .on_press(Message::BoxClicked(url.clone()))
    .on_enter(Message::BoxHovered(parent_url, file_info.id))
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}

fn file_type(file_info: &File) -> String {
    if file_info.is_dir {
        "Directory".to_string()
    } else {
        "File".to_string()
    }
}
