use iced::padding::{bottom, left, top};
use iced::widget::{column, container, horizontal_rule, row, text, vertical_rule};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{color, Element};

pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.
}

pub fn main() -> iced::Result {
    iced::application("MuChat", Application::update, Application::view)
        .theme(Application::theme)
        .run()
}

#[derive(Default)]
struct Application {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Application {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Element::from(column![row![
            container(
                column![
                    container(text("Chats").size(20))
                        .align_x(Center)
                        .align_y(Center)
                        .height(48)
                        .width(Fill),
                    container(horizontal_rule(0.5)).padding(bottom(16)),
                    column![
                        text("Miguel"),
                        text("Hello World!").size(10).color(color!(0xA1A1A1))
                    ]
                    .spacing(4)
                    .padding(left(48))
                    .width(Fill),
                    container(horizontal_rule(0.5)).padding(top(16))
                ]
                .width(300)
            ),
            container(vertical_rule(0.5)).height(Fill),
            container("No selected chat")
                .align_x(Center)
                .align_y(Center)
                .width(Fill)
                .height(Fill)
        ]])
        // .explain(color!(0xff0000))
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}
