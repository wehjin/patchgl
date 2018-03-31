extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::{window, WindowMsg};
use patchgl::flood::{Flood, Length, Padding, Position};
use patchgl::TouchMsg;
use std::sync::mpsc::{channel, Sender};

fn main() {
    let palette = Palette::new();
    window::render_forever(320, 400, move |window| {
        let mut count = 0;
        let (counter, counter_msgs) = channel::<TouchMsg>();
        flood_window(&window, flood_from_count(count, &palette, &counter));
        while let Ok(msg) = counter_msgs.recv() {
            if let TouchMsg::End(x, y) = msg {
                println!("CLICK: {},{}", x, y);
                count = count + 1;
                flood_window(&window, flood_from_count(count, &palette, &counter));
            }
        }
    });
}

fn flood_window(window: &Sender<WindowMsg>, flood: Flood) {
    window.send(WindowMsg::Flood(flood)).unwrap_or(());
}

fn flood_from_count(count: i32, palette: &Palette, counter: &Sender<TouchMsg>) -> Flood {
    let body = Flood::Text(format!("{}", count), palette.text);
    let bottom_bar = {
        let up_button = Flood::Text(String::from("Up"), palette.text)
            + Padding::Dual(Length::Spacing, Length::Spacing / 4)
            + (Flood::Color(palette.button_background) + Padding::Uniform(Length::Spacing / 4))
            + Flood::Color(palette.button_border)
            .track(34, counter.clone());
        up_button
    };
    let before_background = body + (Position::Bottom(Length::FingerTip), bottom_bar) + Padding::Uniform(Length::Spacing);
    (before_background) + Flood::Color(palette.background)
}

struct Palette {
    pub text: Color,
    pub background: Color,
    pub button_background: Color,
    pub button_border: Color,
}

impl Palette {
    fn new() -> Self {
        Palette {
            text: Color::from(X11Color::Indigo),
            background: Color::from(X11Color::Lavender),
            button_background: Color::from(X11Color::Thistle),
            button_border: Color::from(X11Color::MediumPurple),
        }
    }
}

