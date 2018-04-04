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
        let panel_padding = Padding::Uniform(Length::Spacing / 2);
        let panels = vec![
            Flood::Color(Color::red()) + Padding::Behind(Length::Spacing / 4) + panel_padding.clone(),
            Flood::Color(Color::green()) + Padding::Behind(Length::Spacing / 2) + panel_padding.clone(),
            Flood::Color(Color::blue()) + Padding::Behind(Length::Spacing) + panel_padding.clone(),
        ];
        let bar = patchgl::flood::bar(panels);
        let flood = bar
            + Padding::Dual(Length::Spacing, Length::Full / 4)
            + (Stratum::Sub, Flood::Color(Color::white()));
        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}

