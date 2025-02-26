use crate::window::files::Message;

pub fn display_bar<'a>(content: String) -> iced::Element<'a, Message> {
    iced::widget::text_input("Enter file address", &content)
        .on_input(Message::DisplayBarContentChanged)
        .on_submit(Message::DisplayBarContentSubmitted)
        .into()
}
