extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{DirectorMsg, ScreenMsg};
use patchgl::{Block, Color, Sigil, WebColor};
use patchgl::director;
use patchgl::screen;
use std::sync::mpsc::Sender;

fn main() {
    let (width, height) = (320, 400);
    let director = director::spawn((width, height, None), |msg, carry| {
        match msg {
            DirectorMsg::ScreenReady(next_screen) => {
                let (width, height, _) = carry;
                send_block_to_screen(width, height, &next_screen);
                ((width, height, Some(next_screen)), director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenResized(new_width, new_height) => {
                let (_, _, screen) = carry;
                if let Some(ref screen) = screen {
                    send_block_to_screen(new_width, new_height, &screen);
                }
                ((new_width, new_height, screen), director::ScanFlow::Continue)
            }
            DirectorMsg::ScreenClosed => {
                ((0, 0, None), director::ScanFlow::Break)
            }
        }
    });
    screen::start(width, height, director);
}

fn send_block_to_screen(width: u32, height: u32, screen: &Sender<ScreenMsg>) {
    let block = Block {
        sigil: Sigil::Color(Color::from_web(WebColor::Blue)),
        width: width as f32,
        height: height as f32,
        ..Default::default()
    };
    screen.send(ScreenMsg::AddBlock(1, block)).unwrap();
}

