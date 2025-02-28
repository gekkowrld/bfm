use std::path::PathBuf;

use iced::widget::{column, text_editor};
use iced::{Element, Task, window};
use iced::{Length, Subscription};

use crate::config::conf;
use crate::files::fs;
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
            Screen::Files(_, file_path) => fs::path_to_string(file_path),
            Screen::FileDisplay(file_path) => fs::path_to_string(file_path),
            Screen::ErrorDislay(error) => error.clone(),
        }
        .replace("/", " - ")
            + " — bfm file manager"
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::WindowEvent)
    }

    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Welcome,
                display_bar_content: String::new(),
                content: text_editor::Content::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleFullscreen(_mode) => {}
            Message::Edit(action) => {
                println!("{:#?}", action);
                self.content.perform(action);
            }

            Message::Error(error) => {
                self.screen = Screen::ErrorDislay(error);
            }

            Message::OpenFile(file_path) => {
                let content = match fs::file_content(file_path.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        self.screen = Screen::ErrorDislay(err.to_string());
                        return;
                    }
                };

                self.content = text_editor::Content::with_text(&content);
                self.screen = Screen::FileDisplay(file_path);
            }
            Message::BoxHovered(file_path, id) => {
                self.screen = Screen::Files(id, file_path.clone());
                self.display_bar_content = fs::path_to_string(&file_path);
            }

            Message::OpenLink(link_path) => {
                self.screen = Screen::Files("".to_string(), link_path.clone());
                self.display_bar_content = fs::path_to_string(&link_path);
            }

            Message::DisplayBarContentChanged(content) => {
                self.display_bar_content = content;
            }

            Message::DisplayBarContentSubmitted => {
                self.screen =
                    Screen::Files("".to_string(), PathBuf::from(&self.display_bar_content));
            }

            Message::BoxClicked(file_path) => {
                self.screen = Screen::Files("".to_string(), file_path.clone());
                self.display_bar_content = fs::path_to_string(&file_path);
            }

            Message::WindowEvent(event) => {
                if let iced::Event::Window(window_event) = event {
                    match window_event {
                        iced::window::Event::RedrawRequested(pos) => {
                            println!("Requst redraw: {:#?}", pos);
                        }
                        iced::window::Event::Opened { position: _, size } => {
                            let mut width = crate::config::conf::ColumnWidth::default();
                            width.name = size.width / 3.0;
                            width.size = size.width / 3.0;
                            width.type_ = size.width / 3.0;
                            conf::Config::new().set_column_width(&width);
                        }
                        iced::window::Event::Resized(size) => {
                            let mut width = crate::config::conf::ColumnWidth::default();
                            width.name = (size.width / 3.0) - 20.0;
                            width.size = size.width / 3.0;
                            width.type_ = size.width / 3.0;
                            conf::Config::new().set_column_width(&width);
                        }
                        _ => {}
                    }
                }
            }

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
            text_editor(&self.content)
                .placeholder("Loading file...")
                .on_action(Message::Edit)
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
