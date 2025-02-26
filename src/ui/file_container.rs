use iced::Length;
use iced::widget::mouse_area;
use iced::widget::rich_text;
use iced::widget::row;
use iced::widget::span;

use crate::config::config::Config;
use crate::files::file::File;
use crate::window::files::Message;
use std::path::Path;

pub fn box_display<'a>(file_info: File) -> iced::Element<'a, Message> {
    let url = file_info.url.clone();
    let config = Config::new().get_column_width();
    mouse_area(
        row![
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
        .spacing(10)
        .width(Length::Fill)
        .height(50),
    )
    .on_press(Message::BoxClicked(url.clone()))
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
