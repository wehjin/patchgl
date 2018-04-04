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
        let panels = vec![
            Flood::Color(Color::grey()) + Padding::Uniform(Length::Spacing),
            Flood::Color(Color::grey()) + Padding::Uniform(Length::Spacing),
        ];
        let bar = patchgl::flood::bar(panels);
        let flood = bar
            + Padding::Dual(Length::Spacing, Length::Full / 4)
            + (Stratum::Sub, Flood::Color(Color::white()));
        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}

