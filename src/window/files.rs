use std::path::PathBuf;

use iced::widget::{column, text_editor};
use iced::{Element, Task, window};
use iced::{Length, Subscription};

use crate::config::conf;
use crate::fs::file;
use crate::ui::display_bar::display_bar;
use crate::ui::error_page::error_display;
use crate::ui::info::directory_information;
use crate::ui::welcome::welcome_content;

pub struct Window {
    screen: Screen,
    display_bar_content: String,
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleFullscreen(window::Mode),
    OpenLink(PathBuf),
    ButtonPressed(ButtonAction),
    DisplayBarContentChanged(String),
    DisplayBarContentSubmitted,
    BoxClicked(PathBuf),
    WindowEvent(iced::Event),
    BoxHovered(PathBuf, String),
    Edit(text_editor::Action),
    OpenFile(PathBuf),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Welcome,
    Files(String, PathBuf),
    FileDisplay(PathBuf),
    ErrorDislay(String),
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
            Screen::Files(_, file_path) => file::path_to_string(file_path),
            Screen::FileDisplay(file_path) => file::path_to_string(file_path),
            Screen::ErrorDislay(error) => error.clone(),
        }
        .replace("/", " - ")
            + " — bfm file manager"
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::WindowEvent)
    }

    pub fn new() -> (Self, Task<Message>) {
        let mut content = text_editor::Content::new();
        let conf = conf::Config::new();
        let screen = match conf.get_last_path() {
            Some(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    Screen::Files("".to_string(), path)
                } else {
                    let file_content = match file::file_content(path.clone()) {
                        Ok(content) => content,
                        Err(err) => {
                            return (
                                Self {
                                    screen: Screen::ErrorDislay(err.to_string()),
                                    display_bar_content: path.to_string_lossy().to_string(),
                                    content,
                                },
                                Task::none(),
                            );
                        }
                    };
                    content = text_editor::Content::with_text(&file_content);
                    Screen::FileDisplay(path)
                }
            }
            None => Screen::Welcome,
        };
        (
            Self {
                screen,
                display_bar_content: String::new(),
                content,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleFullscreen(_mode) => Task::none(),
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }

            Message::Error(error) => {
                self.screen = Screen::ErrorDislay(error);
                Task::none()
            }

            Message::OpenFile(file_path) => {
                let content = match file::file_content(file_path.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        self.screen = Screen::ErrorDislay(err.to_string());
                        return Task::none();
                    }
                };

                self.content = text_editor::Content::with_text(&content);
                self.screen = Screen::FileDisplay(file_path.clone());
                self.display_bar_content = file::path_to_string(&file_path);
                Task::none()
            }
            Message::BoxHovered(file_path, id) => {
                self.screen = Screen::Files(id, file_path.clone());
                self.display_bar_content = file::path_to_string(&file_path);
                Task::none()
            }

            Message::OpenLink(link_path) => {
                self.screen = Screen::Files("".to_string(), link_path.clone());
                self.display_bar_content = file::path_to_string(&link_path);
                Task::none()
            }

            Message::DisplayBarContentChanged(content) => {
                self.display_bar_content = content;
                Task::none()
            }

            Message::DisplayBarContentSubmitted => {
                self.screen =
                    Screen::Files("".to_string(), PathBuf::from(&self.display_bar_content));
                Task::none()
            }

            Message::BoxClicked(file_path) => {
                self.screen = Screen::Files("".to_string(), file_path.clone());
                self.display_bar_content = file::path_to_string(&file_path);
                Task::none()
            }

            Message::WindowEvent(event) => {
                let window_event = match event {
                    iced::Event::Window(window_event) => window_event,
                    _ => return Task::none(),
                };
                let x: Task<Message> = match window_event {
                    iced::window::Event::Opened { position: _, size } => {
                        let mut width = crate::config::conf::ColumnWidth::default();
                        width.name = (size.width / 3.0) - 50.0;
                        width.size = size.width / 3.0;
                        width.type_ = size.width / 3.0;
                        conf::Config::new().set_column_width(&width);
                        Task::none()
                    }
                    iced::window::Event::Resized(size) => {
                        let mut width = crate::config::conf::ColumnWidth::default();
                        width.name = (size.width / 3.0) - 50.0;
                        width.size = size.width / 3.0;
                        width.type_ = size.width / 3.0;
                        conf::Config::new().set_column_width(&width);
                        Task::none()
                    }

                    iced::window::Event::CloseRequested => {
                        let mut conf = conf::Config::new();

                        if !self.display_bar_content.is_empty() {
                            conf.set_last_path(&self.display_bar_content);
                        }

                        window::get_latest().and_then(window::close)
                    }
                    _ => Task::none(),
                };

                x
            }

            Message::ButtonPressed(_action) => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let screen = match &self.screen {
            Screen::Welcome => self.full_window(welcome_content()),
            Screen::Files(box_id, path) => self.files_content(box_id.to_string(), path),
            Screen::FileDisplay(file_url) => self.display_file(file_url),
            Screen::ErrorDislay(error) => {
                error_display(error.clone(), self.display_bar_content.clone())
            }
        };

        screen
    }

    fn display_file(&self, _: &PathBuf) -> Element<Message> {
        column![
            display_bar(self.display_bar_content.clone()),
            text_editor(&self.content).on_action(Message::Edit)
        ]
        .into()
    }

    fn full_window<'a>(&self, element: Element<'a, Message>) -> Element<'a, Message> {
        column![display_bar(self.display_bar_content.clone()), element]
            .width(Length::Fill)
            .into()
    }

    fn files_content(&self, id: String, path: &PathBuf) -> Element<Message> {
        column![
            display_bar(self.display_bar_content.clone()),
            directory_information(id, PathBuf::from(path))
        ]
        .into()
    }
}
