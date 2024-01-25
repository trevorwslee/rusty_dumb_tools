#![deny(warnings)]
#![allow(unused)]

use iced::executor;
use iced::mouse;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::text;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Text;
use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::Sandbox;
use iced::Settings;

pub fn handle_demo_calc_gui() {
    Counter::run(Settings::default());
}

struct Counter {
    count: i32,
}
#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement,
}
impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter { count: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if true {
            column![
                text(format!("Count: {}", self.count)),
                button("Increment").on_press(CounterMessage::Increment),
                button("Decrement").on_press(CounterMessage::Decrement)
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .into()
        } else if true {
            container(column![
                text(format!("Count: {}", self.count)),
                button("Increment").on_press(CounterMessage::Increment),
                button("Decrement").on_press(CounterMessage::Decrement)
            ])
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
        } else {
            let label = Text::new(format!("Count: {}", self.count));
            let incr = Button::new("Increment").on_press(CounterMessage::Increment);
            let decr = Button::new("Decrement").on_press(CounterMessage::Decrement);
            let col = Column::new().push(incr).push(label).push(decr);
            Container::new(col)
                .center_x()
                .center_y()
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
        }
    }
}

