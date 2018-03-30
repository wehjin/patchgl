extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood;
use patchgl::flood::Flood;


fn main() {
    let flood = Flood::Color(Color::from(X11Color::Thistle));
    flood::render(320, 400, flood);
}

