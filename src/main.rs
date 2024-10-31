use fake::faker::name::raw::*;
use fake::{locales::*, Fake};
use iced::padding::{bottom, left, top};
use iced::widget::{column, container, horizontal_rule, row, text, vertical_rule};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{color, Element};

pub fn main() -> iced::Result {
    iced::application("muchat", Application::update, Application::view)
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
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        home()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}

fn home<'a>() -> Element<'a, Message> {
    let random_name: String = Name(EN).fake();

    let sidebar = container(
        column![
            container(text("Chats").size(20))
                .align_x(Center)
                .align_y(Center)
                .height(48)
                .width(Fill),
            container(horizontal_rule(0.5)).padding(bottom(16)),
            column![
                text(random_name),
                text("Hello World!").size(10).color(color!(0xA1A1A1))
            ]
            .spacing(4)
            .padding(left(48))
            .width(Fill),
            container(horizontal_rule(0.5)).padding(top(16))
        ]
        .width(300),
    );

    let divider = container(vertical_rule(0.5)).height(Fill);

    let basic_text = container("No selected chat")
        .align_x(Center)
        .align_y(Center)
        .width(Fill)
        .height(Fill);

    column![row![sidebar, divider, basic_text]].into()
}
