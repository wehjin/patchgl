extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::Color;
use patchgl::color::argb;
use patchgl::flood::*;
use patchgl::material;
use patchgl::window;
use patchgl::window::WindowMsg;

fn main() {
    window::start(320, 400, |window| {
        let accent_color: Color = material::Color::PinkA200.into();
        let placeholder_color: Color = material::Color::LightBackgroundTextDisabled.into();
        let background_flood = Flood::Color(material::Color::LightBackground.into());

        let text = Flood::Text("Placeholder".into(), placeholder_color, Placement::Start);
        let bottom_line = Flood::Color(accent_color);
        let entry = text +
            (Position::Bottom(Length::Pixels(8.0)), Flood::Color(argb::TRANSPARENT)) +
            (Position::Bottom(Length::Pixels(2.0)), bottom_line) +
            (Position::Bottom(Length::Pixels(8.0)), Flood::Color(argb::TRANSPARENT));

        let flood = entry
            + Padding::Dual(Length::Spacing, Length::Full * 0.4)
            + (Stratum::JustBelow, background_flood);

        window.send(WindowMsg::Flood::<()>(flood)).unwrap();
    });
}

