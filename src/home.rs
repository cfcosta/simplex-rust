use iced::padding::left;
use iced::widget::{column, container, horizontal_rule, mouse_area, row, text, vertical_rule};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{color, Element, Padding};

#[derive(Default)]
struct ChatInfo {
    name: String,
    content_message: String,
}

#[derive(Default)]
pub struct Application {
    state: bool,
    chat_info: ChatInfo,
}

#[derive(Debug, Clone)]
pub enum Event {
    ClickOnChat { name: String, message: String },
}

impl Application {
    pub fn update(&mut self, action: Event) {
        match action {
            Event::ClickOnChat { name, message } => {
                self.state = true;
                self.chat_info = ChatInfo {
                    name,
                    content_message: message,
                };
            }
        }
    }

    pub fn view(&self) -> Element<Event> {
        home(self)
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}

fn home<'a>(app: &Application) -> Element<'a, Event> {
    let name = "Bob";
    let message = "Do you have some bitcoin in your wallet?";

    let sidebar = container(
        column![
            container(text("Chats").size(20))
                .align_x(Center)
                .align_y(Center)
                .height(48)
                .width(Fill),
            container(horizontal_rule(0.5)),
            mouse_area(
                container(
                    column![text(name), text(message).size(10).color(color!(0xA1A1A1)),]
                        .spacing(4)
                        .padding(left(48))
                        .width(Fill)
                )
                .padding(Padding {
                    top: 16.,
                    bottom: 16.,
                    ..Default::default()
                })
            )
            .on_press(Event::ClickOnChat {
                name: name.to_string(),
                message: message.to_string(),
            }),
            container(horizontal_rule(0.5))
        ]
        .width(300),
    );

    let divider = container(vertical_rule(0.5)).height(Fill);

    let main_content = if !app.state {
        container(text("No selected chat"))
            .align_x(Center)
            .align_y(Center)
            .width(Fill)
            .height(Fill)
    } else {
        container(text(format!(
            "\"{}\" from {}",
            app.chat_info.content_message, app.chat_info.name
        )))
        .align_x(Center)
        .align_y(Center)
        .width(Fill)
        .height(Fill)
    };

    column![row![sidebar, divider, main_content]].into()
}
