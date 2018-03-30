extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood;
use patchgl::flood::Flood;


fn main() {
    let flood = Flood::Text(String::from("0"), Color::from(X11Color::Indigo));
    flood::render_forever(320, 400, flood);
}

