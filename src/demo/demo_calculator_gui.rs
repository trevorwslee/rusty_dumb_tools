//! core [`crate::calculator`] sub-demo code

#![deny(warnings)]
#![allow(unused)]

use iced::executor;
use iced::mouse;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
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

pub fn handle_demo_calculator_gui() {
    if true {
        // let labels = vec![
        //     vec!["Label 1", "Label 2"],
        //     vec!["Label 3", "Label 4"],
        // ];

        // let column = Column::new()
        //     .push(Text::new(labels[0][0]))
        //     .push(Text::new(labels[0][1]))
        //     .push(Text::new(labels[1][0]))
        //     .push(Text::new(labels[1][1]));
        // let content = column.into();

        // Run the iced application with the content
        //iced::run(iced::Settings::default(), content);
        CalculatorGUI::run(Settings::default());
    } else {
        Counter::run(Settings::default());
    }
}

struct CalculatorGUI {
    count: i32,
}
#[derive(Debug, Clone, Copy)]
enum CalculatorGUIMessage {
    Increment,
    Decrement,
}
impl Sandbox for CalculatorGUI {
    type Message = CalculatorGUIMessage;

    fn new() -> Self {
        CalculatorGUI { count: 0 }
    }

    fn title(&self) -> String {
        String::from("Calculator GUI")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CalculatorGUIMessage::Increment => self.count += 1,
            CalculatorGUIMessage::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            row![text(format!("Count: {}", self.count)),],
            row![
                button("Increment").on_press(CalculatorGUIMessage::Increment),
                button("Decrement").on_press(CalculatorGUIMessage::Decrement)
            ]
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
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
