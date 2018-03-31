extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood;
use patchgl::flood::{Flood, Length, Position, Thickness};


fn main() {
    let text_color = Color::from(X11Color::Indigo);
    let background_color = Color::from(X11Color::Lavender);
    let button_background_color = Color::from(X11Color::Thistle);
    let button_border_color = Color::from(X11Color::MediumPurple);

    let count = 0;
    {
        let count_flood =
            Flood::Text(format!("{}", count), text_color) - Thickness::Uniform(Length::Padding);

        let button_flood = Flood::Text(String::from("+"), text_color) - Thickness::Horizontal(Length::Padding)
            & (Flood::Color(button_background_color) - Thickness::Uniform(Length::Padding / 4))
            & Flood::Color(button_border_color);

        let full_flood = (count_flood + (Position::BottomBar, button_flood - Thickness::Uniform(Length::Padding)))
            & Flood::Color(background_color);

        flood::render_forever(320, 400, full_flood);
    }
}

