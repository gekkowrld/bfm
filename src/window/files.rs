use std::path::PathBuf;

use iced::widget::column;
use iced::{Element, Task, window};
use iced::{Length, Subscription};

use crate::config::config;
use crate::ui::display_bar::display_bar;
use crate::ui::info::directory_information;
use crate::ui::welcome::welcome_content;

pub struct Window {
    screen: Screen,
    display_bar_content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleFullscreen(window::Mode),
    OpenLink(url::Url),
    ButtonPressed(ButtonAction),
    DisplayBarContentChanged(String),
    DisplayBarContentSubmitted,
    BoxClicked(url::Url),
    WindowEvent(iced::Event),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Welcome,
    Files(url::Url),
}

#[derive(Debug, Clone)]
pub enum ButtonAction {
    NewFolder,
    Delete,
    Rename,
    Copy,
    Paste,
    Cut,
    Quit,
}

impl Window {
    pub fn title(&self) -> String {
        match &self.screen {
            Screen::Welcome => "Welcome".to_owned(),
            Screen::Files(file_url) => file_url.path().to_owned(),
        }
        .replace("/", " - ")
            + " -- bfm file manager"
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::WindowEvent)
    }

    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Welcome,
                display_bar_content: String::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleFullscreen(_mode) => {}

            Message::OpenLink(link_url) => {
                self.screen = Screen::Files(link_url.clone());
                self.display_bar_content = link_url.path().to_owned();
            }

            Message::DisplayBarContentChanged(content) => {
                self.display_bar_content = content;
            }

            Message::DisplayBarContentSubmitted => {
                if let Ok(url) = url::Url::from_directory_path(&self.display_bar_content) {
                    self.screen = Screen::Files(url);
                }
            }

            Message::BoxClicked(url) => {
                self.screen = Screen::Files(url.clone());
                self.display_bar_content = url.path().to_owned();
            }

            Message::WindowEvent(event) => match event {
                iced::Event::Window(window_event) => match window_event {
                    iced::window::Event::RedrawRequested(pos) => {
                        println!("Requst redraw: {:#?}", pos);
                    }
                    iced::window::Event::Opened { position: _, size } => {
                        let mut width = crate::config::config::ColumnWidth::default();
                        width.name = size.width / 3.0;
                        width.size = size.width / 3.0;
                        width.type_ = size.width / 3.0;
                        config::Config::new().set_column_width(&width);
                    }
                    iced::window::Event::Resized(size) => {
                        let mut width = crate::config::config::ColumnWidth::default();
                        width.name = size.width / 3.0;
                        width.size = size.width / 3.0;
                        width.type_ = size.width / 3.0;
                        config::Config::new().set_column_width(&width);
                    }
                    _ => {}
                },
                _ => {}
            },

            Message::ButtonPressed(action) => match action {
                ButtonAction::NewFolder => {
                    println!("New Folder")
                }
                ButtonAction::Delete => {
                    println!("Delete")
                }
                ButtonAction::Rename => {
                    println!("Rename")
                }
                ButtonAction::Copy => {
                    println!("Copy")
                }
                ButtonAction::Paste => {
                    println!("Paste")
                }
                ButtonAction::Cut => {
                    println!("Cut")
                }
                ButtonAction::Quit => {
                    println!("Quit")
                }
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let screen = match &self.screen {
            Screen::Welcome => self.full_window(welcome_content()),
            Screen::Files(path) => self.files_content(path),
        };

        screen
    }

    fn full_window<'a>(&self, element: Element<'a, Message>) -> Element<'a, Message> {
        column![display_bar(self.display_bar_content.clone()), element]
            .width(Length::Fill)
            .into()
    }

    fn files_content(&self, path: &url::Url) -> Element<Message> {
        column![
            display_bar(self.display_bar_content.clone()),
            directory_information(PathBuf::from(path.path()))
        ]
        .into()
    }
}
