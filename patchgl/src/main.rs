#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::{run, RemoteScreen, Block, Quip, Sigil, Color};
use std::thread;
use std::time::Duration;

fn main() {
    run(320, 480, |screen: &RemoteScreen| {
        screen.add_block(1, Block {
            sigil: Sigil::FilledRectangle(Color::blue()),
            width: 320.0,
            height: 320.0,
            approach: 0.0
        });
        screen.add_block(2, Block {
            sigil: Sigil::FilledRectangle(Color::grey()),
            width: 80.0,
            height: 480.0,
            approach: 1.0
        });
        screen.set_quip(Quip {
            text: "I for one welcome our new robot overlords".to_string(),
            line_height: 24.0,
            line_width_max: 320.0
        });
        thread::sleep(Duration::from_secs(10));
        screen.close()
    });
}
