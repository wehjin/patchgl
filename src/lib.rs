extern crate arrayvec;
extern crate cage;
#[macro_use]
extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;
extern crate xml;

pub use base::{Color, WebColor};
use glium::glutin::{ControlFlow, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};
use local_screen::LocalScreen;
pub use remote_director::*;
pub use remote_screen::*;
pub use sigil::Sigil;
use std::marker::Send;


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

#[derive(Clone, Copy)]
pub struct Anchor {
    pub x: f32,
    pub y: f32,
}

impl Anchor {
    pub fn top_left() -> Self { Anchor { x: 0.0, y: 0.0 } }
}

pub struct Block {
    pub sigil: Sigil,
    pub width: f32,
    pub height: f32,
    pub approach: f32,
    pub anchor: Anchor,
}

pub enum ScreenMessage {
    AddBlock(u64, Block),
    Close,
}

pub enum DirectorMessage {}

pub fn start_screen<F>(width: u32, height: u32, on_screen_ready: F) where F: Fn(&RemoteScreen) -> () + Send + 'static
{
    let mut events_loop = EventsLoop::new();
    let mut screen = LocalScreen::new(width, height, &events_loop);
    let events_loop_proxy = events_loop.create_proxy();
    let director = RemoteDirector::connect(events_loop_proxy, on_screen_ready);
    events_loop.run_forever(|ev| {
        match ev {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Closed | WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } => {
                        ControlFlow::Break
                    }
                    WindowEvent::Resized(width, height) => {
                        screen.resize(width, height);
                        ControlFlow::Continue
                    }
                    WindowEvent::Refresh => {
                        screen.draw();
                        ControlFlow::Continue
                    }
                    _ => {
                        ControlFlow::Continue
                    }
                }
            }
            Event::Awakened => {
                while let Some(screen_message) = director.receive_screen_message() {
                    screen.update(screen_message);
                }
                match screen.status() {
                    ScreenStatus::Unchanged => ControlFlow::Continue,
                    ScreenStatus::Changed => {
                        screen.draw();
                        ControlFlow::Continue
                    }
                    ScreenStatus::WillClose => ControlFlow::Break,
                }
            }
            _ => ControlFlow::Continue
        }
    });
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScreenStatus {
    Unchanged,
    Changed,
    WillClose,
}

impl ScreenStatus {
    fn will_close(&self) -> Self {
        ScreenStatus::WillClose
    }
    fn did_change(&self) -> Self {
        if *self == ScreenStatus::WillClose {
            ScreenStatus::WillClose
        } else {
            ScreenStatus::Changed
        }
    }
    fn did_draw(&self) -> Self {
        if *self == ScreenStatus::WillClose {
            ScreenStatus::WillClose
        } else {
            ScreenStatus::Unchanged
        }
    }
}
