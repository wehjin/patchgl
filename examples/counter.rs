extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood;
use patchgl::flood::{Flood, Length, Position, Thickness};


fn main() {
    let count = 0;
    let count_flood = Flood::Text(format!("{}", count), Color::from(X11Color::Indigo))
        - Thickness::Uniform(Length::Padding);

    let button_flood = Flood::Color(Color::from(X11Color::MediumPurple))
        - Thickness::Uniform(Length::Padding);

    let flood = Flood::Barrier(
        Position::BottomMinusLength(Length::FingerTip + Length::Padding * 2),
        Box::new(count_flood),
        Box::new(button_flood),
    );
    flood::render_forever(320, 400, flood);
}

