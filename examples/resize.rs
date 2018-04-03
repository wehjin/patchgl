extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::{window, WindowNote};
use patchgl::flood::Flood;

fn main() {
    window::start(320, 400, |window| {
        let flood = Flood::Color(Color::from(X11Color::Thistle));
        window.send(WindowNote::Flood(flood)).unwrap();
    });
}

