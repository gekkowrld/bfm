use crate::window::ButtonAction;
use crate::window::Message;
use iced::Length;
use iced::widget::{button, column, container, text};

pub fn home_screen<'a>() -> iced::Element<'a, Message> {
    container(
        column![
            text!("Welcome to BF Manager"),
            text!("A File Manager for YOU!"),
            button(text!("Browse Local Files"))
                .on_press(Message::Button(ButtonAction::ListFiles("/".to_string()))),
            button(text!("Browse FTP servers")).on_press(Message::Button(ButtonAction::FTPLogin))
        ]
        .spacing(12),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
