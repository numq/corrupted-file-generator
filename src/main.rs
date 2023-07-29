use iced::{Application, Settings, window};

use crate::navigation::application::Root;

mod elm;
mod navigation;
mod ui;
mod generation;
mod interaction;
mod template;

fn main() -> iced::Result {
    let min_size = (500, 300);
    let settings = Settings {
        window: window::Settings {
            size: min_size,
            min_size: Some(min_size),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    Root::run(settings)
}
