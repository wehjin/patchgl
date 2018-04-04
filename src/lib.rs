extern crate arrayvec;
extern crate cage;
#[macro_use]
extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;
extern crate xml;

pub use anchor::Anchor;
pub use base::{Color, WebColor, X11Color};
pub use block::Block;
pub use sigil::Sigil;
use std::sync::mpsc::Sender;
pub use window::WindowNote;

pub mod material;
pub mod model;
pub mod rendering;
pub mod glyffin;
pub mod base;
pub mod ix;
pub mod parser;
pub mod director;
pub mod screen;
pub mod flood;
pub mod window;
pub mod color;
pub mod app;
pub mod button;
mod sigil;
mod local_screen;
mod anchor;
mod block;

#[derive(Debug)]
pub enum DirectorMsg {
    ScreenReady(Sender<ScreenMsg>),
    ScreenResized(u32, u32),
    ScreenClosed,
    TouchMsg(TouchMsg),
}

#[derive(Debug)]
pub enum ScreenMsg {
    AddBlock(u64, Block),
    Close,
}

#[derive(Copy, Clone, Debug)]
pub enum TouchMsg {
    Begin(u64, f64, f64),
    Cancel(u64),
    Move(u64, f64, f64),
    End(u64, f64, f64),
}

impl TouchMsg {
    pub fn tag(&self) -> u64 {
        match self {
            &TouchMsg::Begin(tag, _, _) => tag,
            &TouchMsg::Cancel(tag) => tag,
            &TouchMsg::Move(tag, _, _) => tag,
            &TouchMsg::End(tag, _, _) => tag,
        }
    }
}


