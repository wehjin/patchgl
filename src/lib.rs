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
pub use sigil::Sigil;
use std::sync::mpsc::Sender;

pub mod model;
pub mod renderer;
pub mod glyffin;
pub mod base;
pub mod ix;
pub mod parser;
pub mod director;
pub mod screen;
mod sigil;
mod local_screen;
mod anchor;
mod block;


#[derive(Debug)]
pub enum DirectorMsg {
    ScreenReady(Sender<ScreenMsg>),
    ScreenResized(u32, u32),
    ScreenClosed,
}

#[derive(Debug)]
pub enum ScreenMsg {
    AddBlock(u64, Block),
    Close,
}

