pub fn button(
    content: &str,
    action: crate::window::files::ButtonAction,
) -> iced::widget::Button<crate::window::files::Message> {
    iced::widget::Button::new(iced::widget::Text::new(content))
        .on_press(crate::window::files::Message::ButtonPressed(action))
}
