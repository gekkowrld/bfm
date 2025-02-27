use iced::Length;
use iced::Theme;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::text;

use crate::config::conf::Config;
use crate::files::file::File;
use crate::ui::file_icon::icon;
use crate::window::files::Message;
use std::path::Path;

pub fn box_display<'a>(
    box_style: fn(&Theme) -> container::Style,
    file_info: File,
) -> iced::Element<'a, Message> {
    let url = file_info.url.clone();
    let config = Config::new().get_column_width();
    let parent_url =
        url::Url::from_directory_path(Path::new(&url.path()).parent().unwrap()).unwrap();

    let file_name: String = match Path::new(url.path()).file_name() {
        Some(name) => name.to_str().unwrap_or("Unnamed File").to_string(),
        None => "Unnamed File".to_string(),
    };

    mouse_area(
        container(
            row![
                icon(file_info.is_dir),
                text!("{file_name}").width(Length::Fixed(config.name)),
                text!("{}", file_type(&file_info)).width(Length::Fixed(config.type_)),
                text!("{}", file_info.file.metadata().unwrap().len().to_string())
                    .width(Length::Fixed(config.type_)),
            ]
            .padding(10)
            .width(Length::Fill),
        )
        .style(box_style),
    )
    .on_press(Message::BoxClicked(url.clone()))
    .on_enter(Message::BoxHovered(parent_url.clone(), file_info.id))
    .on_exit(Message::BoxHovered(parent_url, "".to_string()))
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
