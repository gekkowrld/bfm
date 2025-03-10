use crate::window::ButtonAction;
use crate::window::Message;
use humanize_bytes::humanize_bytes_binary;
use iced::widget::{column, container, mouse_area, row, text};
use iced::{Color, Element, Length, Theme};
use vfs::FileInfo;

pub fn card<'a>(file: &FileInfo) -> Element<'a, Message> {
    let is_dir = file.is_dir;
    let path = file.name.clone();
    let (file_type, got_error) = file_type(file);
    let file_size_color = Color::parse("#ffb8b8").unwrap_or(Color::WHITE);
    let file_type_color = if got_error {
        Color::parse("#8b0000").unwrap_or(Color::WHITE)
    } else {
        Color::parse("#eb5800").unwrap_or(Color::WHITE)
    };

    mouse_area(
        container(
            column![
                text!("{}", path.clone()),
                row![
                    text!("{}", humanize_bytes_binary!(file.size)).color(file_size_color),
                    text!("{file_type}").color(file_type_color),
                ]
                .spacing(10)
            ]
            .padding(10)
            .width(Length::Fill)
            .spacing(10),
        )
        .style(if got_error {
            error_container_style
        } else {
            container_style
        }),
    )
    .on_press(if got_error {
        Message::NOACTION
    } else if is_dir {
        if file.is_ftp {
            Message::Button(ButtonAction::ListFtpFiles(path))
        } else {
            Message::Button(ButtonAction::ListFiles(path))
        }
    } else {
        if file.is_ftp {
            Message::Button(ButtonAction::ViewFtpFile(path))
        } else {
            Message::Button(ButtonAction::ViewFile(path))
        }
    })
    .interaction(if got_error {
        iced::mouse::Interaction::NotAllowed
    } else {
        iced::mouse::Interaction::Pointer
    })
    .into()
}

fn error_container_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(103, 104, 107))),
        text_color: Some(Color::from_rgb8(0xee, 0xee, 0xee)),
        ..container::rounded_box(theme)
    }
}

fn container_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(42, 42, 42))),
        text_color: Some(Color::from_rgb8(0xee, 0xee, 0xee)),
        ..container::rounded_box(theme)
    }
}

fn file_type(file: &FileInfo) -> (String, bool) {
    let mut f_types = String::new();

    if file.is_dir {
        f_types.push_str("Directory");
    } else {
        f_types.push_str("File");
    }

    if file.is_symlink {
        f_types.push_str("  Symlink");
    }

    (f_types, false)
}
