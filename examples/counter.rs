extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::flood::{Flood, Length, Position, Thickness};
use patchgl::TouchMsg;
use patchgl::window;
use std::sync::mpsc::channel;
use std::thread;


fn main() {
    let text_color = Color::from(X11Color::Indigo);
    let background_color = Color::from(X11Color::Lavender);
    let button_background_color = Color::from(X11Color::Thistle);
    let button_border_color = Color::from(X11Color::MediumPurple);

    let count = 0;
    {
        let count_flood =
            Flood::Text(format!("{}", count), text_color) - Thickness::Uniform(Length::Padding);

        let pressable_button_flood = Flood::Text(String::from("+"), text_color) - Thickness::Horizontal(Length::Padding)
            & (Flood::Color(button_background_color) - Thickness::Uniform(Length::Padding / 4))
            & Flood::Color(button_border_color);

        let (tracker, tracker_msgs) = channel::<TouchMsg>();
        thread::spawn(move || {
            while let Ok(msg) = tracker_msgs.recv() {
                if let TouchMsg::End(x, y) = msg {
                    println!("CLICK: {},{}", x, y)
                }
            }
        });

        let button_flood = pressable_button_flood.track(34, tracker);

        let full_flood = (count_flood + (Position::Bottom(Length::BottomBarHeight), button_flood - Thickness::Uniform(Length::Padding)))
            & Flood::Color(background_color);

        window::render_forever(320, 400, full_flood);
    }
}

