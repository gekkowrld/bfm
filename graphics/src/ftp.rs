use crate::window::Message;
use iced::widget::{button, column, container, text_input};
use iced::{Element, Length, Pixels};

pub fn ftp_login(address: String, username: String, password: String) -> Element<'static, Message> {
    container(
        column![
            text_input("Enter Address here", &address).on_input(Message::FTPAdressChanged),
            text_input("Enter Username here", &username).on_input(Message::FTPUsernameChanged),
            text_input("Enter Password here", &password)
                .on_input(Message::FTPPasswordChanged)
                .secure(true),
            button("LOGIN").on_press(Message::Button(
                crate::window::ButtonAction::FTPLoginSubmit(crate::window::FtpLogin::new(
                    password, username, address
                ))
            )),
        ]
        .max_width(Pixels(720.0))
        .spacing(12),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
