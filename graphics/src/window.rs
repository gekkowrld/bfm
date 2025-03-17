use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::widget::text_editor::{Action, Content};
use iced::widget::text_input::{Id, focus};
use iced::window::Settings;
use iced::{Task, window};
use vfs::DirectoryInformation;

pub struct Window {
    screen: Screen,
    address: String,
    username: String,
    password: String,
    opt_path: Option<String>,
    ftp_stream: Option<vfs::FTPStream>,
    display_bar: String,
    text_content: Option<Content>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NOACTION,
    FTPAdressChanged(String),
    FTPUsernameChanged(String),
    FTPPasswordChanged(String),
    Button(ButtonAction),
    DisplayBarContentChanged(String),
    DisplayBarContentSubmitted,
    TextEditorAction(Action),
    Event(iced::Event),
}

pub enum Screen {
    Home,
    Local(String),
    ViewFile(String),
    ViewFtpFile,
    ViewFTP(DirectoryInformation),
    FTPLogin,
}

#[derive(Debug, Clone)]
pub enum ButtonAction {
    ListFiles(String),
    ListFtpFiles(String),
    ViewFile(String),
    ViewFtpFile(String),
    FTPLogin,
    FTPLoginSubmit(FtpLogin),
}

#[derive(Debug, Clone)]
pub struct FtpLogin {
    pub password: String,
    pub username: String,
    pub address: String,
}

impl FtpLogin {
    pub fn new(password: String, username: String, address: String) -> Self {
        Self {
            password,
            username,
            address,
        }
    }
}

impl Window {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home,
                address: String::new(),
                username: String::new(),
                password: String::new(),
                ftp_stream: None,
                opt_path: None,
                display_bar: String::new(),
                text_content: None,
            },
            Task::none(),
        )
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

    pub fn title(&self) -> String {
        let app_name = String::from("BF Manager — ");
        match &self.screen {
            Screen::Home => format!("{app_name}Home"),
            Screen::Local(path) => format!("{app_name}{}", Self::path_to_title(path)),
            Screen::ViewFile(path) => format!("{app_name}{}", Self::path_to_title(path)),
            Screen::FTPLogin => format!("{app_name}FTP Login"),
            Screen::ViewFTP(dir) => format!("{app_name}{}", Self::path_to_title(&dir.name)),
            Screen::ViewFtpFile => format!(
                "{app_name}{}",
                Self::path_to_title(self.opt_path.as_ref().unwrap_or(&String::new()))
            ),
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
            Message::Event(event) => match event {
                iced::Event::Keyboard(key) => match key {
                    iced::keyboard::Event::KeyReleased {
                        key,
                        location: _,
                        modifiers: _,
                    } => match key {
                        iced::keyboard::Key::Character(key_code) => {
                            if key_code.as_str() == "/" {
                                focus(Id::new("display_bar"))
                            } else {
                                return Task::none();
                            }
                        }
                        _ => Task::none(),
                    },
                    _ => Task::none(),
                },
                _ => Task::none(),
            },
            Message::TextEditorAction(action) => {
                // Disable editing for now
                match action {
                    Action::Edit(_) => return Task::none(),
                    _ => (),
                }

                self.text_content.as_mut().unwrap().perform(action);
                Task::none()
            }
            Message::FTPAdressChanged(address) => {
                self.address = address;
                Task::none()
            }
            Message::DisplayBarContentChanged(value) => {
                self.display_bar = value;
                Task::none()
            }
            Message::DisplayBarContentSubmitted => {
                let ftp_stream = match self.ftp_stream {
                    Some(_) => self.ftp_stream.as_mut(),
                    None => None,
                };

                self.screen = path_to_screen(self.display_bar.to_string(), ftp_stream);
                Task::none()
            }
            Message::FTPUsernameChanged(username) => {
                self.username = username;
                Task::none()
            }
            Message::FTPPasswordChanged(password) => {
                self.password = password;
                Task::none()
            }
            Message::Button(action) => match action {
                ButtonAction::ListFiles(file) => {
                    self.screen = Screen::Local(file);
                    Task::none()
                }
                ButtonAction::ListFtpFiles(path) => {
                    let ftp_stream = match self.ftp_stream {
                        Some(_) => self.ftp_stream.as_mut(),
                        None => None,
                    };

                    self.screen = list_ftp_files(ftp_stream, Some(path));

                    Task::none()
                }

                ButtonAction::ViewFtpFile(file) => {
                    self.opt_path = Some(file.clone());
                    if let Some(ftp_stream) = &mut self.ftp_stream {
                        let content = vfs::read_file(vfs::FS::FTP(ftp_stream), &file);
                        let content = match content {
                            Ok(content) => content,
                            Err(err) => {
                                println!("{err}");
                                return Task::none();
                            }
                        };
                        let content = String::from_utf8_lossy(&content.content);
                        self.text_content = Some(Content::with_text(&content));
                        self.screen = Screen::ViewFtpFile;
                    } else {
                        println!("FTP stream is not connected.");
                    }

                    Task::none()
                }
                ButtonAction::ViewFile(file) => {
                    self.screen = Screen::ViewFile(file.clone());
                    self.opt_path = Some(file.clone());
                    let content = vfs::read_file(vfs::FS::Local, &file).unwrap().content;

                    let content = String::from_utf8_lossy(&content);
                    self.text_content = Some(Content::with_text(content.to_string().as_str()));
                    Task::none()
                }
                ButtonAction::FTPLogin => {
                    self.screen = Screen::FTPLogin;
                    Task::none()
                }
                ButtonAction::FTPLoginSubmit(login) => {
                    self.ftp_stream =
                        vfs::connect(&login.address, &login.username, &login.password);

                    let ftp_stream = match self.ftp_stream {
                        Some(_) => self.ftp_stream.as_mut(),
                        None => None,
                    };

                    self.screen = list_ftp_files(ftp_stream, None);

                    Task::none()
                }
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let view_screen = match &self.screen {
            Screen::Home => crate::home::home_screen(),
            Screen::Local(path) => crate::dir::directory(path),
            Screen::ViewFile(_path) => crate::text_viewer::display_file(
                self.opt_path.as_ref().unwrap_or(&"".to_string()),
                self.text_content.as_ref().unwrap(),
            ),
            Screen::ViewFTP(dir) => crate::dir::directory_info(dir),
            Screen::ViewFtpFile => crate::text_viewer::display_file(
                self.opt_path.as_ref().unwrap_or(&"".to_string()),
                self.text_content.as_ref().unwrap(),
            ),
            Screen::FTPLogin => crate::ftp::ftp_login(
                self.address.clone(),
                self.username.clone(),
                self.password.clone(),
            ),
        };

        iced::widget::Column::new()
            .push(crate::display_bar::display_bar(self.display_bar.clone()))
            .push(view_screen)
            .into()
    }
}

fn list_ftp_files(ftp: Option<&mut vfs::FTPStream>, path: Option<String>) -> Screen {
    let ftp_stream = match ftp {
        Some(ftp) => ftp,
        None => return Screen::FTPLogin,
    };

    let path = match path {
        Some(path) => path,
        None => match ftp_stream.pwd() {
            Ok(path) => path,
            Err(err) => {
                println!("Error getting path {err}");
                return Screen::FTPLogin;
            }
        },
    };

    let director_info = vfs::list_files(vfs::FS::FTP(ftp_stream), &path);
    Screen::ViewFTP(director_info.unwrap())
}

fn path_to_screen(path: String, ftp: Option<&mut vfs::FTPStream>) -> Screen {
    let bar_c = crate::display_bar::display_bar_content(path.clone());

    match bar_c.fs {
        crate::display_bar::BarFS::Local => Screen::Local(bar_c.path),
        crate::display_bar::BarFS::FTP => {
            let ftp_stream = match ftp {
                Some(ftp) => ftp,
                None => return Screen::FTPLogin,
            };

            let director_info = vfs::list_files(vfs::FS::FTP(ftp_stream), &bar_c.path);
            let info = match director_info {
                Ok(info) => info,
                Err(err) => {
                    println!("{err}");
                    return Screen::Home;
                }
            };

            Screen::ViewFTP(info)
        }
    }
}
