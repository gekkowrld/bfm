use crate::window::ButtonAction;
use crate::window::Message;
use iced::widget::{column, container, mouse_area, text};
use iced::{Color, Element, Length, Theme};
use std::path::PathBuf;

pub fn card<'a>(file: &PathBuf) -> Element<'a, Message> {
    let is_dir = file.is_dir();
    let path = file.to_str().unwrap().to_owned();
    mouse_area(
        container(
            column![
                text!("{}", path.clone()),
                text!("{}", if is_dir { "Directory" } else { "File" })
            ]
            .padding(10)
            .width(Length::Fill)
            .spacing(10),
        )
        .style(container_style),
    )
    .on_press(if is_dir {
        Message::Button(ButtonAction::ListFiles(path))
    } else {
        Message::Button(ButtonAction::ViewFile(path))
    })
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}

fn container_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(42, 42, 42))),
        text_color: Some(Color::from_rgb8(0xee, 0xee, 0xee)),
        ..container::rounded_box(theme)
    }
}
