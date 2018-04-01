extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use app::App;
use app::Palette;
use patchgl::Color;
use patchgl::flood::{Flood, Length, Padding, Position, Touching};
use patchgl::TouchMsg;
use patchgl::window;
use std::sync::mpsc::Sender;

mod channel_adapter;
mod app;

fn main() {
    window::show(320, 400, |window| {
        let app = App::new(update, draw);
        app.run(Model::default(), window);
    });
}

fn update(model: &mut Model, msg: AppMsg) {
    match msg {
        AppMsg::Press(code) => {
            model.active_code = Some(code);
        }
        AppMsg::Cancel(code) => {
            if model.active_code == Some(code) {
                model.active_code = None;
            }
        }
        AppMsg::Release(code) => {
            if model.active_code == Some(code) {
                match code {
                    UP_CODE => model.count += 1,
                    DOWN_CODE => model.count -= 1,
                    RESET_CODE => model.count = 0,
                    _ => (),
                }
                model.active_code = None
            }
        }
        AppMsg::Ignore => ()
    }
}

#[derive(Default, Debug)]
struct Model {
    pub count: i32,
    pub active_code: Option<u64>,
}

impl Model {
    pub fn count(&self) -> i32 { self.count }
}

enum AppMsg {
    Press(u64),
    Cancel(u64),
    Release(u64),
    Ignore,
}

impl From<TouchMsg> for AppMsg {
    fn from(touch_msg: TouchMsg) -> Self {
        match touch_msg {
            TouchMsg::Begin(code, _, _) => AppMsg::Press(code),
            TouchMsg::Cancel(code) => AppMsg::Cancel(code),
            TouchMsg::Move(_, _, _) => AppMsg::Ignore,
            TouchMsg::End(code, _, _) => AppMsg::Release(code),
        }
    }
}

const UP_CODE: u64 = 32;
const DOWN_CODE: u64 = 33;
const RESET_CODE: u64 = 34;


fn draw(model: &Model, palette: &Palette, app: &Sender<AppMsg>) -> Flood {
    let touch_watcher: Sender<TouchMsg> = channel_adapter::spawn(app, AppMsg::from);
    let text = format!("{}", model.count());
    let body = Flood::Text(text, palette.text);
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

                let interactive_button = button + Touching::Channel(code, touch_watcher.clone());
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
    enabled_raised_button(label, palette.text, palette.button_idle_background)
}

fn depressed_enabled_raised_button_from_palette(label: &str, palette: &Palette) -> Flood {
    enabled_raised_button(label, palette.text, palette.button_activated_background)
}

fn enabled_raised_button(label: &str, text_color: Color, background_color: Color) -> Flood {
    let text = Flood::Text(String::from(label).to_uppercase(), text_color);
    let text_padding = Padding::Dual(Length::Spacing, Length::Full / 4);
    let background = Flood::Color(background_color);
    text + text_padding + background
}

