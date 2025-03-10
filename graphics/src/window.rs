use iced::advanced::graphics::image::image_rs::ImageFormat;
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
}

#[derive(Debug, Clone)]
pub enum Message {
    NOACTION,
    FTPAdressChanged(String),
    FTPUsernameChanged(String),
    FTPPasswordChanged(String),
    Button(ButtonAction),
}

pub enum Screen {
    Home,
    Local(String),
    ViewFile(String),
    ViewFtpFile(String),
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
            Screen::FTPLogin => format!("{app_name}FTP Login"),
            Screen::ViewFTP(dir) => format!("{app_name}{}", Self::path_to_title(&dir.name)),
            Screen::ViewFtpFile(_) => format!(
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
            Message::FTPAdressChanged(address) => {
                self.address = address;
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
                    if let Some(ftp_stream) = &mut self.ftp_stream {
                        match vfs::list_files(vfs::FS::FTP(ftp_stream), &path) {
                            Ok(dir) => {
                                self.screen = Screen::ViewFTP(dir);
                            }
                            Err(e) => {
                                println!("Failed to list files: {:?}", e);
                            }
                        }
                    } else {
                        println!("FTP stream is not connected.");
                    }

                    Task::none()
                }

                ButtonAction::ViewFtpFile(file) => {
                    self.opt_path = Some(file.clone());
                    if let Some(ftp_stream) = &mut self.ftp_stream {
                        match vfs::read_file(vfs::FS::FTP(ftp_stream), &file) {
                            Ok(file) => {
                                self.screen = Screen::ViewFtpFile(file.content);
                            }
                            Err(e) => {
                                println!("Failed to read file: {:?}", e);
                            }
                        }
                    } else {
                        println!("FTP stream is not connected.");
                    }

                    Task::none()
                }
                ButtonAction::ViewFile(file) => {
                    self.screen = Screen::ViewFile(file);
                    Task::none()
                }
                ButtonAction::FTPLogin => {
                    self.screen = Screen::FTPLogin;
                    Task::none()
                }
                ButtonAction::FTPLoginSubmit(login) => {
                    self.ftp_stream =
                        vfs::connect(&login.address, &login.username, &login.password);

                    if let Some(ftp_stream) = &mut self.ftp_stream {
                        match vfs::list_files(vfs::FS::FTP(ftp_stream), "/") {
                            Ok(dir) => {
                                self.screen = Screen::ViewFTP(dir);
                            }
                            Err(e) => {
                                println!("Failed to list files: {:?}", e);
                            }
                        }
                    } else {
                        println!("FTP stream is not connected.");
                    }

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
            Screen::ViewFTP(dir) => crate::dir::directory_info(dir),
            Screen::ViewFtpFile(content) => crate::text_viewer::file_display(content.to_string()),
            Screen::FTPLogin => crate::ftp::ftp_login(
                self.address.clone(),
                self.username.clone(),
                self.password.clone(),
            ),
        }
    }
}
