//! The graphics lib to provide the GUI for the application.

use iced::Font;

mod dir;
mod display_bar;
mod file_card;
mod ftp;
mod home;
mod text_viewer;
mod window;

/// The result type for the graphics lib.
pub type IcedResult = iced::Result;

/// Run the graphics lib.
pub fn run() -> IcedResult {
    iced::application(
        window::Window::title,
        window::Window::update,
        window::Window::view,
    )
    .antialiasing(true)
    .decorations(true)
    .default_font(Font::MONOSPACE)
    .theme(window::Window::theme)
    .window(window::Window::window_settings())
    .run_with(window::Window::new)
}
