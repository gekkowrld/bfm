use crate::ui::display_bar::display_bar;
use crate::ui::files::Message;
use iced::{Element, widget::column};

pub fn error_display(error: String, bar_content: String) -> Element<'static, Message> {
    column![display_bar(bar_content), iced::widget::Text::new(error)].into()
}
