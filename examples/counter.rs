extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood;
use patchgl::flood::Flood;


fn main() {
    let count = 0;
    let flood = Flood::Text(format!("{}", count), Color::from(X11Color::Indigo));
    flood::render_forever(320, 400, flood);
}

