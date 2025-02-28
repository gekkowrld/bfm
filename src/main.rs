use bfm::window::files::Window;

pub fn main() -> iced::Result {
    iced::application(Window::title, Window::update, Window::view)
        .antialiasing(true)
        .resizable(true)
        .decorations(true)
        .subscription(Window::subscription)
        .run_with(Window::new)
}
