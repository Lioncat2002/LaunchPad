use common::{AppData, GlobalCommand};
use iced::widget::{column, scrollable, text};
use iced::widget::{text_input, Column};
use iced::Alignment::Center;
use iced::Task;

use super::loader::load;

#[derive(Default)]
pub struct WindowState {
    apps: Vec<AppData>,
    query: String,
    commands:Vec<GlobalCommand>
}

#[derive(Debug, Clone)]
pub enum Message {
    Init((Vec<AppData>,Vec<GlobalCommand>)),
    ContentChanged(String),
    ContentSubmit,
}

impl WindowState {
    pub fn new() -> (Self, Task<Message>) {
        (
            WindowState {
                query: "".to_string(),
                apps: vec![],
                commands:vec![]
            },
            Task::perform(load(), Message::Init),
        )
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Init(tuple) => {
                self.apps = tuple.0;
                self.commands=tuple.1;
            }
            
            Message::ContentChanged(content) => {
                self.query = content;
                let result = search::similarity_search(&self.apps, &self.query);
                self.apps = result;
            }
            Message::ContentSubmit => {
                commands::parse_command(&self.query, self.apps.first());
            }
            
        }
    }

    pub fn view(&self) -> Column<Message> {
        let mut result_columns = column![];
        for app in &self.apps {
            result_columns = result_columns.push(text!("{}", app.name.as_str()).width(700));
        }
        column![
            text_input("Search...", &self.query)
                .on_input(Message::ContentChanged)
                .on_submit(Message::ContentSubmit),
            scrollable(result_columns.width(700).padding(20)),
        ]
        .padding(20)
        .align_x(Center)
    }
}
