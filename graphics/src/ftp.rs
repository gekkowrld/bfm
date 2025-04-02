use crate::window::Message;
use iced::widget::{button, column, container, text_input};
use iced::{Element, Length, Pixels};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ExampleFtp {
    pub address: &'static str,
    pub username: &'static str,
    pub password: &'static str,
    message: Option<&'static str>,
}

impl std::fmt::Display for ExampleFtp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Just show the address
        f.write_str(
            format!(
                "Address {} {}",
                self.address,
                match self.message {
                    Some(msg) => format!("({msg})"),
                    None => "".to_string(),
                }
            )
            .as_str(),
        )
    }
}

pub fn ftps() -> Vec<ExampleFtp> {
    // This are publicly available ftp services,
    // Please don't take this as invitation to DDos the servers
    vec![
        ExampleFtp {
            address: "ftp.scene.org",
            username: "demo",
            password: "demo@example.org",
            message: None,
        },
        ExampleFtp {
            address: "ftp.gnu.org",
            username: "anonymous",
            password: "password",
            message: Some("May be closed in later date"),
        },
    ]
}

pub fn ftp_login(address: String, username: String, password: String) -> Element<'static, Message> {
    container(
        column![
            text_input("Enter Address here (e.g ftp.scene.org)", &address)
                .on_input(Message::FTPAdressChanged),
            text_input("Enter Username here (e.g ftp)", &username)
                .on_input(Message::FTPUsernameChanged),
            text_input("Enter Password here (e.g email@example.com)", &password)
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
