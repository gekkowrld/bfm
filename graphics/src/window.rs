use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::window::Settings;
use iced::{Task, window};

pub struct Window {
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum Message {
    NOACTION,
    Button(ButtonAction),
}

#[derive(Debug)]
pub enum Screen {
    Home,
    Local(String),
    ViewFile(String),
}

#[derive(Debug, Clone)]
pub enum ButtonAction {
    ListFiles(String),
    ViewFile(String),
}

impl Window {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        let app_name = String::from("BF Manager — ");
        match &self.screen {
            Screen::Home => format!("{app_name}Home"),
            Screen::Local(path) => format!("{app_name}{}", Self::path_to_title(path)),
            Screen::ViewFile(path) => format!("{app_name}{}", Self::path_to_title(path)),
        }
    }

    fn path_to_title(path: &str) -> String {
        let mut title = String::new();

        for part in Self::split_path(path.trim_matches('/')) {
            title.push_str(&part);
            title.push_str(" ::> ");
        }

        title
    }

    fn split_path(path: &str) -> Vec<String> {
        path.split('/').map(|s| s.to_string()).collect()
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Nord
    }

    pub fn window_settings() -> Settings {
        Settings {
            icon: window::icon::from_file_data(
                include_bytes!("../../assets/logo/bfm.png"),
                Some(ImageFormat::Png),
            )
            .ok(),
            ..window::Settings::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NOACTION => Task::none(),
            Message::Button(action) => match action {
                ButtonAction::ListFiles(file) => {
                    self.screen = Screen::Local(file);
                    Task::none()
                }
                ButtonAction::ViewFile(file) => {
                    self.screen = Screen::ViewFile(file);
                    Task::none()
                }
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home => crate::home::home_screen(),
            Screen::Local(path) => crate::dir::directory(path),
            Screen::ViewFile(path) => crate::text_viewer::file(path.clone()),
        }
    }
}
