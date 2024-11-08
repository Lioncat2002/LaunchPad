mod core;
use core::core::WindowState;
use std::any::Any;
use std::process::Command;

use iced::widget::{button, column, text, Column};
use iced::window::Settings;
use iced::Center;

pub fn main() -> iced::Result {
    let apps = installed::list().unwrap();
    
    for app in apps {
        // metadata accessor fns, these are only evaluated when used
        let name = app.name();
        let dump=app.dump();
        let version = app.version();
        let publisher = app.publisher();
        
        println!("{name} v{version} by {publisher}");

        
    }
    iced::application("A cool counter", WindowState::update,WindowState::view)
        .window(Settings {
            decorations: false,
            size: iced::Size {
                width: 720f32,
                height: 320f32,
            },
            position: iced::window::Position::Centered,
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
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

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
