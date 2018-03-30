extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, WebColor};
use patchgl::flood;
use patchgl::flood::Flood;


fn main() {
    let flood = Flood::Color(Color::from(WebColor::Thistle));
    flood::render(320, 400, flood);
}

