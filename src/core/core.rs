use std::process::Command;

use iced::keyboard;
use iced::widget::{text_input, Column};
use iced::widget::{button, column, text};
use iced::Alignment::Center;

#[derive(Default)]
pub struct WindowState{
    data: String
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    ContentSubmit
}

impl WindowState{
    pub fn update(&mut self,message: Message){
        
        match message{
            Message::ContentChanged(content)=>{
                self.data=content;
            },
            Message::ContentSubmit=>{
                let apps = installed::list().unwrap();
                for app in apps {
                    // metadata accessor fns, these are only evaluated when used
                    let name = app.name();
                    let dump=app.dump();
                    if name==self.data{
                        if let Some(start) = dump.find("DisplayIcon: ") {
                            // Offset by length of "DisplayIcon: "
                            let start = start + "DisplayIcon: ".len();
                            // Find the end of the path by looking for the next newline
                            if let Some(end) = dump[start..].find('\n') {
                                println!("{}",dump[start..start + end].trim().to_string());
                                Command::new(dump[start..start + end].trim().replace('"', "").to_string()).spawn().unwrap();
                            }
                        }
                        break;
                    }
            
                    
                    
                    
                }
            }
        }
    }

    pub fn view(&self)->Column<Message>{
        column![
            text_input("Search...", &self.data).on_input(Message::ContentChanged).on_submit(Message::ContentSubmit)
        ]
        .padding(20)
        .align_x(Center)
    }
}