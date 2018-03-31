extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::{window, WindowMsg};
use patchgl::flood::Flood;

fn main() {
    window::render_forever(320, 400, |window| {
        let flood = Flood::Color(Color::from(X11Color::Thistle));
        window.send(WindowMsg::Flood(flood)).unwrap();
    });
}

