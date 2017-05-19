#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;
extern crate rusttype;
extern crate arrayvec;

use patchgllib::{run, RemoteScreen, Block, Sigil, Anchor, Color};
use std::thread;
use std::time::Duration;

fn main() {
    run(320, 480, |screen: &RemoteScreen| {
        screen.add_block(1, Block {
            sigil: Sigil::FilledRectangle(Color::blue()),
            width: 300.0,
            height: 320.0,
            approach: 0.0,
            anchor: Anchor::top_left()
        });
        screen.add_block(2, Block {
            sigil: Sigil::FilledRectangle(Color::grey()),
            width: 80.0,
            height: 480.0,
            approach: 0.01,
            anchor: Anchor::top_left()
        });
        screen.add_block(3, Block {
            sigil: Sigil::Paragraph { line_height: 24.0, text: "I for one welcome our new robot overlords".to_string() },
            width: 320.0,
            height: 480.0,
            approach: 0.02,
            anchor: Anchor::top_left()
        });
        thread::sleep(Duration::from_secs(40));
        screen.close()
    });
}
