use bfm::ui::files::Window;
use iced::Font;

pub fn main() -> iced::Result {
    iced::application(Window::title, Window::update, Window::view)
        .antialiasing(true)
        .resizable(true)
        .decorations(true)
        .theme(Window::theme)
        .subscription(Window::subscription)
        .default_font(Font::MONOSPACE)
        .exit_on_close_request(false)
        .run_with(Window::new)
}
