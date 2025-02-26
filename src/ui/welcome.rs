use crate::ui::link::link;
use crate::window::files::Message;
use iced::Element;
use iced::widget::{Column, Text};

pub fn welcome_content<'a>() -> Element<'a, Message> {
    let mut column = Column::new().spacing(20);

    let title = Text::new("Welcome to bfm").size(50);

    let title = title.width(iced::Length::Fill).center();

    let link = link(
        "Home".to_string(),
        url::Url::from_directory_path("/home/gekkowrld/").unwrap(),
    );

    column = column.push(title).push(link);
    column.into()
}
