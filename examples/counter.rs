extern crate arrayvec;
extern crate cage;
extern crate patchgl;
extern crate rusttype;
extern crate xml;

use app::App;
use app::Palette;
use patchgl::flood::*;
use patchgl::TouchMsg;
use patchgl::window;
use std::sync::Arc;

mod app;
mod button;

fn main() {
    window::show(640, 400, |window| {
        let app = App::new(update, draw);
        app.run(Model::default(), window);
    });
}

fn update(model: &mut Model, msg: AppMsg) {
    match msg {
        AppMsg::Press(code) => {
            update_buttons(model, code, |active_code, &(button_code, _)| {
                if button_code == active_code {
                    Some(button::Msg::Press)
                } else {
                    Some(button::Msg::Unpress)
                }
            });
        }
        AppMsg::Cancel(code) => {
            update_buttons(model, code, |active_code, &(button_code, _)| {
                if button_code == active_code {
                    Some(button::Msg::Unpress)
                } else {
                    None
                }
            });
        }
        AppMsg::Release(code) => {
            update_buttons(model, code, |active_code, &(button_code, _)| {
                if button_code == active_code {
                    Some(button::Msg::Release)
                } else {
                    None
                }
            });
        }
        AppMsg::ButtonNotes(button_notes) => {
            button_notes.iter().for_each(|&note| {
                update(model, AppMsg::ButtonNote(note.to_owned()));
            });
        }
        AppMsg::ButtonNote((button_code, button_note)) => {
            match button_note {
                button::Note::Clicked => {
                    match button_code {
                        UP_CODE => model.count += 1,
                        DOWN_CODE => model.count -= 1,
                        RESET_CODE => model.count = 0,
                        _ => (),
                    }
                }
            }
        }
        AppMsg::Ignore => ()
    }
}

fn update_buttons<F>(model: &mut Model, active_code: u64, get_msg: F) where
    F: Fn(u64, &(u64, button::Mdl)) -> Option<button::Msg>
{
    let mut notes = Vec::<(u64, button::Note)>::new();
    let mut buttons = Vec::<(u64, button::Mdl)>::new();
    model.buttons.iter()
         .for_each(|coded_button| {
             let &(button_code, ref button_mdl) = coded_button;
             let mut mdl = button_mdl.clone();
             if let Some(msg) = get_msg(active_code, coded_button) {
                 if let Some(note) = button::update(&mut mdl, msg) {
                     notes.push((button_code, note));
                 }
             }
             buttons.push((button_code, mdl));
         });
    model.buttons = buttons;
    update(model, AppMsg::ButtonNotes(notes));
}


#[derive(Debug)]
struct Model {
    pub count: i32,
    pub buttons: Vec<(u64, button::Mdl)>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            count: 0,
            buttons: vec![
                (DOWN_CODE, button::Mdl::colored_flat("Down")),
                (RESET_CODE, button::Mdl::plain_flat("Reset")),
                (UP_CODE, button::Mdl::colored_flat("Up")),
            ],
        }
    }
}

const UP_CODE: u64 = 32;
const DOWN_CODE: u64 = 33;
const RESET_CODE: u64 = 34;

enum AppMsg {
    Press(u64),
    Cancel(u64),
    Release(u64),
    ButtonNotes(Vec<(u64, button::Note)>),
    ButtonNote((u64, button::Note)),
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

fn draw(model: &Model, palette: &Palette) -> Flood<AppMsg> {
    let edge_padding = Padding::Uniform(Length::Spacing);
    let background = Flood::Color(palette.light_background);
    let special = button::special(palette);

    let text = format!("{:+}", model.count);
    let body = Flood::Text(text, palette.primary, Placement::Center);
    let bottom_bar = {
        let enumerated = model.buttons.iter().enumerate().collect::<Vec<_>>();
        let empty_bar = Flood::Color(palette.light_background);
        let bar = enumerated.into_iter().fold(empty_bar, |bar, (i, &(code, ref button_mdl))| {
            let segment = {
                let button = button::draw(button_mdl, palette);
                let sensor = Sensor::Touch(code, Arc::new(|touch_msg| {
                    AppMsg::from(touch_msg)
                }));
                let segment_padding = Padding::Horizontal(Length::Spacing / 4);
                button + sensor + segment_padding
            };
            bar + (Position::Right(Length::Full / (i as u32 + 1)), segment)
        });
        bar
    };
    body + (Position::Bottom(Length::FingerTip), bottom_bar) + edge_padding + (Position::Bottom(Length::FingerTip), special) + background
}
