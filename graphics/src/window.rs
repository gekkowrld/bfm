use iced::Task;

pub struct Window {}

#[derive(Debug)]
pub enum Message {
    Open,
}

impl Window {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Nord
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Open => {
                println!("Open");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        iced::widget::Column::new()
            .push(iced::widget::Text::new("Hello, world!"))
            .into()
    }
}
