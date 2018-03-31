extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use patchgl::{Color, X11Color};
use patchgl::{window, WindowMsg};
use patchgl::flood::{Flood, Length, Padding, Position, Touching};
use patchgl::TouchMsg;
use std::sync::mpsc::{channel, Sender};

const UP_CODE: u64 = 32;
const DOWN_CODE: u64 = 33;
const RESET_CODE: u64 = 34;

enum AppMsg {
    Add,
    Subtract,
    Reset,
    None,
}

fn main() {
    let palette = Palette::new();
    fn update(count: i32, msg: AppMsg) -> i32 {
        match msg {
            AppMsg::Add => count + 1,
            AppMsg::Subtract => count - 1,
            AppMsg::Reset => 0,
            AppMsg::None => count,
        }
    }

    window::render_forever(320, 400, move |window| {
        let mut count = 0;
        let (app, app_msgs) = channel::<TouchMsg>();
        flood_window(&window, draw(count, &palette, &app));
        while let Ok(msg) = app_msgs.recv() {
            if let TouchMsg::End(code, _x, _y) = msg {
                let msg = match code {
                    UP_CODE => AppMsg::Add,
                    DOWN_CODE => AppMsg::Subtract,
                    RESET_CODE => AppMsg::Reset,
                    _ => AppMsg::None,
                };
                count = update(count, msg);
                flood_window(&window, draw(count, &palette, &app));
            }
        }
    });
}

fn flood_window(window: &Sender<WindowMsg>, flood: Flood) {
    window.send(WindowMsg::Flood(flood)).unwrap_or(());
}

fn draw(count: i32, palette: &Palette, app: &Sender<TouchMsg>) -> Flood {
    let body = Flood::Text(format!("{}", count), palette.text);
    let bottom_bar = {
        let up_button = button(palette, app, "Up", UP_CODE) + Padding::Horizontal(Length::Spacing / 4);
        let down_button = button(palette, app, "Down", DOWN_CODE) + Padding::Horizontal(Length::Spacing / 4);
        let reset_button = button(palette, app, "Reset", RESET_CODE) + Padding::Horizontal(Length::Spacing / 4);
        down_button + (Position::Right(Length::Half), up_button) + (Position::Right(Length::Third), reset_button)
    };
    let before_background = body + (Position::Bottom(Length::FingerTip), bottom_bar) + Padding::Uniform(Length::Spacing);
    (before_background) + Flood::Color(palette.background)
}

fn button(palette: &Palette, counter: &Sender<TouchMsg>, name: &str, code: u64) -> Flood {
    let button = Flood::Text(String::from(name), palette.text)
        + Padding::Dual(Length::Spacing, Length::Spacing/2 )
        + (Flood::Color(palette.button_background) + Padding::Uniform(Length::Spacing / 4))
        + Flood::Color(palette.button_border)
        + Touching::Channel(code, counter.clone());
    button
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

