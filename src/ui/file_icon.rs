use crate::fs::embed::Assets;
use crate::window::files::Message;
use iced::{Element, Length, widget::svg};

pub fn icon<'a>(is_dir: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(Assets::get_icon(if is_dir {
        "dir"
    } else {
        "file"
    })))
    .width(Length::Fill)
    /* .style(move |_, _| svg::Style { color: Some(color) }) */
    .into()
}

#[cfg(test)]
mod tests {
    use crate::fs::embed::Assets;

    #[test]
    fn test_get_icon() {
        let icon = Assets::get_icon("dir");
        assert_eq!(icon.len(), 303);
    }
}
