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

fn draw(count: i32, palette: &Palette, watcher: &Sender<TouchMsg>) -> Flood {
    let body = Flood::Text(format!("{}", count), palette.text);
    let bottom_bar = {
        let button_data = vec![("Up", UP_CODE), ("Down", DOWN_CODE), ("Reset", RESET_CODE)];
        let enumerated = button_data.into_iter().enumerate().collect::<Vec<_>>();
        let bar_background = Flood::Color(palette.background);
        let bar = enumerated.into_iter().fold(bar_background, |bar, (i, (label, code))| {
            let segment_padding = Padding::Horizontal(Length::Spacing / 4);
            let segment = {
                let idle_button = idle_raised_enabled_button_from_palette(label, palette);
                let interactive_button = idle_button + Touching::Channel(code, watcher.clone());
                interactive_button + segment_padding
            };
            bar + (Position::Right(Length::Full / (i as u32 + 1)), segment)
        });
        bar
    };
    let before_background = body + (Position::Bottom(Length::FingerTip), bottom_bar) + Padding::Uniform(Length::Spacing);
    (before_background) + Flood::Color(palette.background)
}

fn idle_raised_enabled_button_from_palette(label: &str, palette: &Palette) -> Flood {
    let button = raised_enabled_button(label, palette.text, palette.button_idle_background, palette.button_border);
    button
}

fn raised_enabled_button(label: &str, text_color: Color, background_color: Color, border_color: Color) -> Flood {
    let text = Flood::Text(String::from(label), text_color);
    let text_padding = Padding::Dual(Length::Spacing, Length::Spacing / 2);
    let background = Flood::Color(background_color) + Padding::Uniform(Length::Spacing / 4);
    let border = Flood::Color(border_color);
    text + text_padding + background + border
}

struct Palette {
    pub text: Color,
    pub background: Color,
    pub button_idle_background: Color,
    pub button_border: Color,
}

impl Palette {
    fn new() -> Self {
        Palette {
            text: Color::from(X11Color::Indigo),
            background: Color::from(X11Color::Lavender),
            button_idle_background: Color::from(X11Color::Thistle),
            button_border: Color::from(X11Color::MediumPurple),
        }
    }
}

