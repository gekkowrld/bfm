use iced::Element;
use iced::widget::text_editor;

use crate::window::Message;

pub fn display_file(content: &text_editor::Content) -> Element<Message> {
    text_editor(content)
        .on_action(Message::TextEditorAction)
        .placeholder("CONTENT?!")
        .into()
}
