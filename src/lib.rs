extern crate arrayvec;
extern crate cage;
#[macro_use]
extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;
extern crate xml;


pub use anchor::Anchor;
pub use base::{Color, WebColor};
pub use block::Block;
use local_screen::LocalScreen;
pub use remote_director::RemoteDirector;
pub use remote_screen::RemoteScreen;
pub use sigil::Sigil;
use std::marker::Send;
use std::sync::mpsc;

pub mod model;
pub mod renderer;
pub mod glyffin;
pub mod base;
pub mod ix;
pub mod parser;
mod sigil;
mod remote_screen;
mod remote_director;
mod local_screen;
mod anchor;
mod block;

pub enum ScreenMessage {
    AddBlock(u64, Block),
    Close,
}

pub enum DirectorMessage {}

pub trait ScreenRunner {
    fn on_screen_ready(&mut self, screen: RemoteScreen);
}

pub fn create_screen<T: ScreenRunner + Send + 'static>(width: u32, height: u32, screen_runner: T) {
    let (screen_message_sender, screen_message_receiver) = mpsc::channel::<ScreenMessage>();
    let (director_message_sender, director_message_receiver) = mpsc::channel::<DirectorMessage>();
    std::thread::spawn(move || {
        let mut screen_runner = screen_runner;
        screen_runner.on_screen_ready(RemoteScreen::new(screen_message_sender, director_message_receiver));
    });

    let remote_director = RemoteDirector {
        _director_message_sender: director_message_sender,
        screen_message_receiver,
    };
    LocalScreen::start(width, height, remote_director);
}
