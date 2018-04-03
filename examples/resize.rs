extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood::Flood;
use patchgl::window;
use patchgl::window::WindowMsg;

fn main() {
    window::start(320, 400, |window| {
        let flood = Flood::Color(Color::from(X11Color::Thistle));
        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}

