pub fn link(
    content: String,
    point_to: url::Url,
) -> iced::widget::Button<'static, crate::window::files::Message> {
    iced::widget::Button::new(iced::widget::Text::new(content))
        .on_press(crate::window::files::Message::OpenLink(point_to))
}
