use human_repr::HumanCount;
use iced::Length;
use iced::Theme;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::text;

use crate::config::conf::Config;
use crate::fs::file::File;
use crate::ui::file_icon::icon;
use crate::window::files::Message;

pub fn box_display<'a>(
    box_style: fn(&Theme) -> container::Style,
    file_info: &File,
) -> iced::Element<'a, Message> {
    let config = Config::new().get_column_width();

    let file_name: String = match file_info.path.file_name() {
        Some(name) => name.to_str().unwrap_or("Unnamed File").to_string(),
        None => "Unnamed File".to_string(),
    };

    mouse_area(
        container(
            row![
                icon(file_info.path.is_dir()),
                text!("{file_name}").width(Length::Fixed(config.name)),
                text!("{}", file_type(file_info)).width(Length::Fixed(config.type_)),
                text!(
                    "{}",
                    file_info.file.metadata().unwrap().len().human_count_bytes()
                )
                .width(Length::Fixed(config.type_)),
            ]
            .padding(10)
            .width(Length::Fill),
        )
        .style(box_style),
    )
    .on_press(if file_info.path.is_dir() {
        Message::BoxClicked(file_info.path.clone())
    } else {
        Message::OpenFile(file_info.path.clone())
    })
    .on_enter(Message::BoxHovered(
        file_info.path.clone().parent().unwrap().to_path_buf(),
        file_info.id.clone(),
    ))
    .on_exit(Message::BoxHovered(
        file_info.path.parent().unwrap().to_path_buf(),
        "".to_string(),
    ))
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}

fn file_type(file_info: &File) -> String {
    if file_info.path.is_dir() {
        "Directory".to_string()
    } else {
        "File".to_string()
    }
}
