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
    Press(u64),
    Cancel(u64),
    Release(u64),
    Ignore,
}

#[derive(Default, Debug)]
struct Model {
    count: i32,
    active_code: Option<u64>,
}

impl Model {
    pub fn count(&self) -> i32 { self.count }
    pub fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Press(code) => {
                self.active_code = Some(code);
            }
            AppMsg::Cancel(code) => {
                if self.active_code == Some(code) {
                    self.active_code = None;
                }
            }
            AppMsg::Release(code) => {
                if self.active_code == Some(code) {
                    match code {
                        UP_CODE => self.count += 1,
                        DOWN_CODE => self.count -= 1,
                        RESET_CODE => self.count = 0,
                        _ => (),
                    }
                    self.active_code = None
                }
            }
            AppMsg::Ignore => ()
        }
    }
}

fn main() {
    let palette = Palette::new();
    window::render_forever(320, 400, move |window| {
        let (app, app_msgs) = channel::<TouchMsg>();
        let mut model = Model::default();
        window.send(WindowMsg::Flood(draw(&model, &palette, &app))).unwrap_or(());
        while let Ok(touch_msg) = app_msgs.recv() {
            let msg = match touch_msg {
                TouchMsg::Begin(code, _, _) => AppMsg::Press(code),
                TouchMsg::Cancel(code) => AppMsg::Cancel(code),
                TouchMsg::Move(_, _, _) => AppMsg::Ignore,
                TouchMsg::End(code, _, _) => AppMsg::Release(code),
            };
            model.update(msg);
            window.send(WindowMsg::Flood(draw(&model, &palette, &app))).unwrap_or(());
        }
    });
}

fn draw(model: &Model, palette: &Palette, watcher: &Sender<TouchMsg>) -> Flood {
    let count = model.count();
    let body = Flood::Text(format!("{}", count), palette.text);
    let bottom_bar = {
        let button_data = vec![("Up", UP_CODE), ("Down", DOWN_CODE), ("Reset", RESET_CODE)];
        let enumerated = button_data.into_iter().enumerate().collect::<Vec<_>>();
        let bar_background = Flood::Color(palette.background);
        let bar = enumerated.into_iter().fold(bar_background, |bar, (i, (label, code))| {
            let segment_padding = Padding::Horizontal(Length::Spacing / 4);
            let segment = {
                let button = if model.active_code == Some(code) {
                    depressed_enabled_raised_button_from_palette(label, palette)
                } else {
                    released_enabled_raised_button_from_palette(label, palette)
                };
                let interactive_button = button + Touching::Channel(code, watcher.clone());
                interactive_button + segment_padding
            };
            bar + (Position::Right(Length::Full / (i as u32 + 1)), segment)
        });
        bar
    };
    let before_background = body + (Position::Bottom(Length::FingerTip), bottom_bar) + Padding::Uniform(Length::Spacing);
    (before_background) + Flood::Color(palette.background)
}

fn released_enabled_raised_button_from_palette(label: &str, palette: &Palette) -> Flood {
    enabled_raised_button(label, palette.text, palette.button_idle_background, palette.button_border)
}

fn depressed_enabled_raised_button_from_palette(label: &str, palette: &Palette) -> Flood {
    enabled_raised_button(label, palette.text, palette.button_activated_background, palette.button_border)
}

fn enabled_raised_button(label: &str, text_color: Color, background_color: Color, border_color: Color) -> Flood {
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
    pub button_activated_background: Color,
    pub button_border: Color,
}

impl Palette {
    fn new() -> Self {
        Palette {
            text: Color::from(X11Color::Indigo),
            background: Color::from(X11Color::Lavender),
            button_idle_background: Color::from(X11Color::Lavender),
            button_activated_background: Color::from(X11Color::Thistle),
            button_border: Color::from(X11Color::MediumPurple),
        }
    }
}

