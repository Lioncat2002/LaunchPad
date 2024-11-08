mod core;
use core::core::WindowState;
use iced::window::Settings;

pub fn main() -> iced::Result {
    
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
        .run_with(WindowState::new)
}
