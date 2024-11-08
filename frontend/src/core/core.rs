use iced::widget::{column, scrollable, text};
use iced::widget::{text_input, Column};
use iced::Alignment::Center;
use iced::Task;

#[derive(Default)]
pub struct WindowState {
    installed_apps: Vec<String>,
    result_apps: Vec<String>,
    data: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Init(Vec<String>),
    ContentChanged(String),
    ContentSubmit,
}

impl WindowState {
    async fn load_apps() -> Vec<String> {
        let apps = installed::list().unwrap();
        let mut loaded_apps = vec![];
        for app in apps {
            let name = app.name().into_owned();
            if name!=""{
                loaded_apps.push(name);
            }
            
        }
        loaded_apps
    }
    pub fn new() -> (Self, Task<Message>) {
        (
            WindowState {
                data: "".to_string(),
                installed_apps: vec![],
                result_apps: vec![],
            },
            Task::perform(WindowState::load_apps(), Message::Init),
        )
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Init(apps) => {
                self.installed_apps = apps.clone();
                self.result_apps = apps;
            }
            Message::ContentChanged(content) => {
                self.data = content;
                let result = search::similarity_search(&self.installed_apps, &self.data);
                self.result_apps = result;
            }
            Message::ContentSubmit => {

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
        for app in &self.result_apps {
            result_columns = result_columns.push(text!("{}", app.as_str()).width(700));
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
