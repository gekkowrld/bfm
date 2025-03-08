//! The graphics lib to provide the GUI for the application.

use iced::Font;

mod window;

/// The result type for the graphics lib.
pub type IcedResult = iced::Result;

/// Run the graphics lib.
pub fn run() -> IcedResult {
    iced::application("BF Manager", window::Window::update, window::Window::view)
        .antialiasing(true)
        .decorations(true)
        .default_font(Font::MONOSPACE)
        .theme(window::Window::theme)
        .run_with(window::Window::new)
}
