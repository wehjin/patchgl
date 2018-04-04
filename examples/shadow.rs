extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::Color;
use patchgl::flood::*;
use patchgl::window;
use patchgl::window::WindowMsg;

fn main() {
    window::start(320, 400, |window| {
        let flood = Flood::Color(Color::grey())
            + Padding::Uniform(Length::Full / 4)
            + (Stratum::Sub, Flood::Color(Color::white()));
        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}

