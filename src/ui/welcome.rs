use crate::window::files::Message;
use iced::Element;
use iced::widget::{Column, Text};

pub fn welcome_content<'a>() -> Element<'a, Message> {
    let mut column = Column::new().spacing(20);

    let title = Text::new("Welcome to bfm").size(50);

    let title = title.width(iced::Length::Fill).center();

    column = column.push(title);
    column.into()
}
