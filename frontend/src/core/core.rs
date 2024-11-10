use std::process::Command;

use common::AppData;
use iced::widget::{column, scrollable, text};
use iced::widget::{text_input, Column};
use iced::Alignment::Center;
use iced::Task;

#[derive(Default)]
pub struct WindowState {
    apps: Vec<AppData>,
    data: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Init(Vec<AppData>),
    ContentChanged(String),
    ContentSubmit,
}

impl WindowState {
    async fn load_apps() -> Vec<AppData> {
        let apps = installed::list().unwrap();
        let mut loaded_apps = vec![];
        for app in apps {
            let name = app.name().into_owned();
            let mut path=String::new();
            if name!=""{
                let dump = app.dump();
                if let Some(start) = dump.find("DisplayIcon: ") {
                    // Offset by length of "DisplayIcon: "
                    let start = start + "DisplayIcon: ".len();
                    // Find the end of the path by looking for the next newline
                    if let Some(end) = dump[start..].find('\n') { 
                           path= dump[start..start + end].trim().replace('"', "").to_string()
                    }
                }
                loaded_apps.push(AppData{
                    name,
                    path,
                });
            }
            
        }
        loaded_apps
    }
    pub fn new() -> (Self, Task<Message>) {
        (
            WindowState {
                data: "".to_string(),
                apps: vec![],
            },
            Task::perform(WindowState::load_apps(), Message::Init),
        )
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Init(apps) => {
                self.apps = apps;
            }
            Message::ContentChanged(content) => {
                self.data = content;
                let result = search::similarity_search(&self.apps, &self.data);
                self.apps = result;
            }
            Message::ContentSubmit => {
                let app=self.apps.get(0);
                match app {
                    Some(data) => {
                        println!("app path: {}",&data.path);
                        Command::new(&data.path).spawn().expect("failed to launch app");
                    },
                    None => println!("Error path for app not found"),
                }
                /*let apps = installed::list().unwrap();
                for app in apps {
                    // metadata accessor fns, these are only evaluated when used
                    let name = app.name();
                    let dump = app.dump();
                    if name == self.data {
                        if let Some(start) = dump.find("DisplayIcon: ") {
                            // Offset by length of "DisplayIcon: "
                            let start = start + "DisplayIcon: ".len();
                            // Find the end of the path by looking for the next newline
                            if let Some(end) = dump[start..].find('\n') {
                                println!("{}", dump[start..start + end].trim().to_string());
                                Command::new(
                                    dump[start..start + end].trim().replace('"', "").to_string(),
                                )
                                .spawn()
                                .unwrap();
                            }
                        }
                        break;
                    }
                }*/
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        let mut result_columns = column![];
        for app in &self.apps {
            result_columns = result_columns.push(text!("{}", app.name.as_str()).width(700));
        }
        column![
            text_input("Search...", &self.data)
                .on_input(Message::ContentChanged)
                .on_submit(Message::ContentSubmit),
                scrollable(
                    result_columns.width(700).padding(20)
                ),
             
        ]
        .padding(20)
        .align_x(Center)
    }
}
